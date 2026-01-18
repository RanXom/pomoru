use std::time::{ Duration, Instant };
use crossterm::event::{ self, Event, KeyCode };
use crossterm::terminal::{ enable_raw_mode, disable_raw_mode };
use std::io::{ stdout, Write };

pub struct Pomo {
    pub time_remaining: Duration,
    pub is_running: bool,
}

impl Pomo {
    pub fn new() -> Self {
        Self {
            time_remaining: Duration::from_secs(25 * 60),
            is_running: false,
        }
    }

    pub fn tick(&mut self) {
        if self.is_running && self.time_remaining.as_secs() > 0 {
            self.time_remaining -= Duration::from_secs(1);
        } else if self.time_remaining.as_secs() == 0 {
            self.is_running = false;
        }
    }

    pub fn toggle_timer(&mut self) {
        self.is_running = !self.is_running;
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;

        self.time_remaining = Duration::from_secs(10);

        let mut last_tick = Instant::now();

        while self.time_remaining > Duration::ZERO {
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char(' ') {
                        self.toggle_timer();
                    }
                }
            }

            if self.is_running && last_tick.elapsed() >= Duration::from_secs(1) {
                println!("{}", self.time_remaining.as_secs());
                self.tick();
                last_tick = Instant::now();
            }
        }

        println!("Done");
        disable_raw_mode()?;
        Ok(())
    }
}
