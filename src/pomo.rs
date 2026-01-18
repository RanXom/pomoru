use std::{ thread, time::Duration };

pub struct Pomo {
    pub time_remaining: Duration,
}

impl Pomo {
    pub fn new() -> Self {
        Self {
            time_remaining: Duration::from_secs(25 * 60),
        }
    }

    pub fn tick(&mut self) {
        self.time_remaining -= Duration::from_secs(1);
    }

    pub fn run(&mut self) {
        self.time_remaining = Duration::from_secs(10);
        while self.time_remaining > Duration::from_secs(0) {
            println!("{}", self.time_remaining.as_secs());
            self.tick();
            thread::sleep(Duration::from_secs(1));
        }
        println!("Done");
    }
}
