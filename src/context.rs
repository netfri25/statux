use std::sync::{Arc, Mutex, Weak};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::Notify;
use tokio::task;

use crate::component::{Component, EMPTY_OUTPUT, MIN_UPDATE_TIME};

#[derive(Default)]
pub struct Context {
    outputs: Vec<Arc<Mutex<String>>>,
    notify: Arc<Notify>,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_output(&mut self) -> Weak<Mutex<String>> {
        let output = Arc::new(Mutex::new(String::from(EMPTY_OUTPUT)));
        let weak = Arc::downgrade(&output);
        self.outputs.push(output);
        weak
    }

    pub fn add_timed(&mut self, interval: Duration, mut component: impl Component + Send + 'static) -> &mut Self {
        let output = self.create_output();
        let notify = self.notify.clone();
        task::spawn(async move {
            let mut next_update;
            loop {
                let mut temp = String::new();
                component.update(&mut temp).await;
                let Some(output) = output.upgrade() else {
                    break
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
        task::spawn(async move {
            let mut temp = String::new();
            component.update(&mut temp).await;
            let Some(output) = output.upgrade() else {
                return
            };
            *output.lock().unwrap() = temp;
        });

        self
    }

    pub async fn run(&mut self) {
        let mut output = String::new();
        loop {
            tokio::time::sleep(MIN_UPDATE_TIME).await;
            self.notify.notified().await;

            output.clear();
            for comp in self.outputs.iter() {
                output.push_str(comp.lock().unwrap().as_str());
                output.push(' ');
            }

            println!("{}", output)
        }
    }
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
            break
        }

        let delta = time.duration_since(now).unwrap_or_default();

        tokio::time::sleep(MIN_UPDATE_TIME.min(delta)).await;
    }
}
