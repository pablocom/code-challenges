//! Print in Order (LeetCode 1114) — three sections must run first → second →
//! third no matter which threads start when.
//!
//! The C# version used two semaphores. Rust's standard library has no
//! semaphore, so each handoff is a one-shot [`Gate`]: a `Mutex<bool>` flag with
//! a `Condvar` to block waiters until the flag is opened.

use std::sync::{Condvar, Mutex};

/// A latch that opens exactly once; waiters block until it does.
#[derive(Default)]
struct Gate {
    opened: Mutex<bool>,
    ready: Condvar,
}

impl Gate {
    fn open(&self) {
        *self.opened.lock().unwrap() = true;
        self.ready.notify_all();
    }

    fn wait(&self) {
        let mut opened = self.opened.lock().unwrap();
        while !*opened {
            opened = self.ready.wait(opened).unwrap();
        }
    }
}

#[derive(Default)]
pub struct PrecedenceSync {
    after_first: Gate,
    after_second: Gate,
}

impl PrecedenceSync {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn first(&self, print_first: impl FnOnce()) {
        print_first();
        self.after_first.open();
    }

    pub fn second(&self, print_second: impl FnOnce()) {
        self.after_first.wait();
        print_second();
        self.after_second.open();
    }

    pub fn third(&self, print_third: impl FnOnce()) {
        self.after_second.wait();
        print_third();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::thread;

    fn run_with_start_order(starts: [char; 3]) -> Vec<&'static str> {
        let sync = PrecedenceSync::new();
        let order: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

        thread::scope(|scope| {
            let sync = &sync;
            let order = &order;
            for section in starts {
                scope.spawn(move || match section {
                    '1' => sync.first(|| order.lock().unwrap().push("first")),
                    '2' => sync.second(|| order.lock().unwrap().push("second")),
                    _ => sync.third(|| order.lock().unwrap().push("third")),
                });
            }
        });

        order.into_inner().unwrap()
    }

    #[test]
    fn prints_in_order_when_started_in_reverse() {
        assert_eq!(
            run_with_start_order(['3', '2', '1']),
            vec!["first", "second", "third"]
        );
    }

    #[test]
    fn prints_in_order_for_every_start_order() {
        for starts in [
            ['1', '2', '3'],
            ['1', '3', '2'],
            ['2', '1', '3'],
            ['2', '3', '1'],
            ['3', '1', '2'],
            ['3', '2', '1'],
        ] {
            assert_eq!(
                run_with_start_order(starts),
                vec!["first", "second", "third"]
            );
        }
    }
}
