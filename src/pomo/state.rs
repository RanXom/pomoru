use ratatui::widgets::ListState;
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
pub enum SessionMode {
    Work,
    ShortBreak,
    LongBreak,
}

pub struct Task {
    pub title: String,
    pub is_done: bool,
}

pub struct Pomo {
    pub mode: SessionMode,
    pub time_remaining: Duration,
    pub is_running: bool,
    pub tasks: Vec<Task>,
    pub task_state: ListState,
    pub should_quit: bool,
}

impl Pomo {
    pub fn new() -> Self {
        Self {
            mode: SessionMode::Work,
            time_remaining: Duration::from_secs(25 * 60),
            is_running: false,
            tasks: Vec::new(),
            task_state: ListState::default(),
            should_quit: false,
        }
    }

    pub fn tick(&mut self) {
        if self.is_running && self.time_remaining.as_secs() > 0 {
            self.time_remaining -= Duration::from_secs(1);
        } else if self.time_remaining.as_secs() == 0 {
            self.is_running = false;
            // logic for switching modes will go here
        }
    }

    pub fn toggle_timer(&mut self) {
        self.is_running = !self.is_running;
    }
}
