use ratatui::widgets::ListState;
use ratatui::prelude::Color;
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
pub enum SessionMode {
    Work,
    ShortBreak,
    LongBreak,
}

impl SessionMode {
    pub fn get_color(&self) -> Color {
        match self {
            SessionMode::Work => Color::Cyan,
            SessionMode::ShortBreak => Color::Green,
            SessionMode::LongBreak => Color::Magenta,
        }
    }
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
        let dummy_tasks = vec![
            Task { title: "Complete Milestone 1".into(), is_done: true },
            Task { title: "Refactor UI for Flocus look".into(), is_done: false },
        ];

        Self {
            mode: SessionMode::Work,
            time_remaining: Duration::from_secs(25 * 60),
            is_running: false,
            tasks: dummy_tasks,
            task_state: ListState::default(),
            should_quit: false,
        }
    }

    pub fn tick(&mut self) {
        if self.is_running && self.time_remaining.as_secs() > 0 {
            self.time_remaining -= Duration::from_secs(1);
        } else if self.time_remaining.as_secs() == 0 {
            self.is_running = false;
            // TODO: Add further mode switching logic
        }
    }

    pub fn toggle_timer(&mut self) {
        self.is_running = !self.is_running;
    }
}
