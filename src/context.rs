use std::sync::{Arc, Mutex, Weak};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use nix::libc::{SIGRTMIN, c_int};
use tokio::signal::unix as signal;
use tokio::sync::Notify;
use tokio::{select, task};
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, PropMode};
use x11rb::rust_connection::RustConnection;
use x11rb::wrapper::ConnectionExt;

use crate::component::{Component, EMPTY_OUTPUT, MIN_UPDATE_TIME};

pub struct Context {
    outputs: Vec<Arc<Mutex<String>>>,
    tasks: Vec<task::JoinHandle<()>>,
    notify: Arc<Notify>,
    conn: RustConnection,
}

impl Context {
    pub fn new() -> Self {
        let outputs = Default::default();
        let tasks = Default::default();
        let notify = Default::default();
        let (conn, _) = RustConnection::connect(None).unwrap();
        Self { outputs, tasks, notify, conn }
    }

    fn create_output(&mut self) -> Weak<Mutex<String>> {
        let output = Arc::new(Mutex::new(String::from(EMPTY_OUTPUT)));
        let weak = Arc::downgrade(&output);
        self.outputs.push(output);
        weak
    }

    pub fn add_timed_signal(&mut self, signal_num: u8, interval: Duration, mut component: impl Component + Send + 'static) -> &mut Self {
        let kind = custom_signal(signal_num);
        let mut handler = signal::signal(kind).unwrap();

        let output = self.create_output();
        let notify = self.notify.clone();
        self.spawn(async move {
            let mut next_update;
            loop {
                let mut temp = String::new();
                component.update(&mut temp).await;
                let Some(output) = output.upgrade() else {
                    break;
                };
                *output.lock().unwrap() = temp;
                notify.notify_one();
                next_update = next_system_time(interval);
                select! {
                    _ = handler.recv() => {}
                    _ = sleep_until(next_update) => {}
                }
            }
        });

        self
    }

    pub fn add_timed(
        &mut self,
        interval: Duration,
        mut component: impl Component + Send + 'static,
    ) -> &mut Self {
        let output = self.create_output();
        let notify = self.notify.clone();
        self.spawn(async move {
            let mut next_update;
            loop {
                let mut temp = String::new();
                component.update(&mut temp).await;
                let Some(output) = output.upgrade() else {
                    break;
                };
                *output.lock().unwrap() = temp;
                notify.notify_one();
                next_update = next_system_time(interval);
                sleep_until(next_update).await;
            }
        });

        self
    }

    pub fn seperator(&mut self) -> &mut Self {
        self.add_static(())
    }

    /// adds a component that only updates once
    pub fn add_static(&mut self, mut component: impl Component + Send + 'static) -> &mut Self {
        let output = self.create_output();
        self.spawn(async move {
            let mut temp = String::new();
            component.update(&mut temp).await;
            let Some(output) = output.upgrade() else {
                return;
            };
            *output.lock().unwrap() = temp;
        });

        self
    }

    fn spawn(&mut self, fut: impl Future<Output = ()> + Send + 'static) {
        let handle = task::spawn(fut);
        self.tasks.push(handle);
    }

    fn update(&self, name: &str) {
        let screens = &self.conn.setup().roots;
        for screen in screens {
            self.conn.change_property8(
                PropMode::REPLACE,
                screen.root,
                AtomEnum::WM_NAME,
                AtomEnum::STRING,
                name.as_bytes(),
            ).ok();
        }

        self.conn.flush().ok();
    }

    fn cleanup(&mut self) {
        self.tasks.iter().for_each(|task| task.abort());
        self.update("");
    }

    pub async fn run(&mut self) {
        let mut output = String::new();
        loop {
            tokio::time::sleep(MIN_UPDATE_TIME).await;
            select! {
                _ = self.notify.notified() => {},
                _ = tokio::signal::ctrl_c() => {
                    self.cleanup();
                    break;
                }
            }

            output.clear();
            for comp in self.outputs.iter() {
                output.push_str(comp.lock().unwrap().as_str());
                output.push(' ');
            }

            self.update(&output[..output.len().checked_sub(1).unwrap_or_default()]);
        }
    }
}

fn custom_signal(signal: u8) -> signal::SignalKind {
    signal::SignalKind::from_raw(SIGRTMIN() + signal as c_int)
}

fn next_system_time(interval: Duration) -> SystemTime {
    let now = SystemTime::now();
    let elapsed = now.duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let interval = interval.as_nanos();

    let next = elapsed.next_multiple_of(interval);

    let secs = (next / 1_000_000_000) as u64;
    let nanos = (next % 1_000_000_000) as u32;
    UNIX_EPOCH + Duration::new(secs, nanos)
}

async fn sleep_until(time: SystemTime) {
    loop {
        let now = SystemTime::now();
        if now >= time {
            break;
        }

        let delta = time.duration_since(now).unwrap_or_default();

        tokio::time::sleep(MIN_UPDATE_TIME.min(delta)).await;
    }
}
