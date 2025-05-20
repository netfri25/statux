use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::component::Component;

#[derive(Default)]
pub struct Executer<'a> {
    timed: BinaryHeap<Reverse<Timed>>,
    items: Vec<Item<'a>>,
}

struct Item<'a> {
    component: Box<dyn Component + 'a>,
    output: String,
}

impl<'a> Executer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, item: Item<'a>) -> usize {
        let index = self.items.len();
        self.items.push(item);
        index
    }

    pub fn add_timed(&mut self, interval: Duration, component: impl Component + 'a) -> &mut Self {
        let component = Box::new(component);
        let index = self.insert(Item {
            component,
            output: String::new(),
        });

        let next_update = SystemTime::now();
        self.timed.push(Reverse(Timed {
            index,
            interval,
            next_update,
        }));
        self
    }

    /// adds a component that only updates once
    pub fn add_static(&mut self, mut component: impl Component) -> &mut Self {
        let mut output = String::new();
        component.update(&mut output);

        // no need to store the component anymore, so using a Box<()> to not allocate anything
        let component = Box::new(());
        self.insert(Item { component, output });
        self
    }

    pub fn run(&mut self) {
        let mut changed = false;
        loop {
            let now = SystemTime::now();
            let Some(mut timed) = self
                .timed
                .peek_mut()
                .filter(|item| now >= item.0.next_update)
            else {
                if changed {
                    self.update_changes();
                    changed = false;
                }

                std::thread::yield_now();
                continue;
            };

            let item = &mut self.items[timed.0.index];
            item.component.update(&mut item.output);

            timed.0.calc_next();
            changed = true;
        }
    }

    fn update_changes(&self) {
        let mut output = String::new();
        for item in self.items.iter() {
            output.push_str(item.output.as_str());
            output.push(' ');
        }

        // TODO: change this to update the xroot name
        println!("{}", output);
    }
}

struct Timed {
    index: usize,
    interval: Duration,
    next_update: SystemTime,
}

impl Timed {
    fn calc_next(&mut self) {
        let now = SystemTime::now();
        let elapsed = now.duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let interval = self.interval.as_nanos();

        // adding 1 to prevent it from returing itself
        let next = (elapsed + 1).next_multiple_of(interval);

        let secs = (next / 1_000_000_000) as u64;
        let nanos = (next % 1_000_000_000) as u32;
        self.next_update = UNIX_EPOCH + Duration::new(secs, nanos);
    }
}

impl PartialEq for Timed {
    fn eq(&self, other: &Self) -> bool {
        self.next_update == other.next_update
    }
}

impl Eq for Timed {}

impl PartialOrd for Timed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timed {
    fn cmp(&self, other: &Self) -> Ordering {
        self.next_update.cmp(&other.next_update)
    }
}
