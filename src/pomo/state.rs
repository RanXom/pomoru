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
    pub show_help: bool
}

pub struct Command {
    pub key: &'static str,
    pub desc: &'static str,
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
            show_help: false
        }
    }

    pub fn tick(&mut self) {
        if self.is_running && self.time_remaining.as_secs() > 0 {
            self.time_remaining -= std::time::Duration::from_secs(1);
        } else if self.is_running && self.time_remaining.as_secs() == 0 {
            self.is_running = false;
            // Simple rotation: Work -> ShortBreak -> Work
            match self.mode {
                SessionMode::Work => {
                    self.mode = SessionMode::ShortBreak;
                    self.time_remaining = std::time::Duration::from_secs(5 * 60);
                }
                _ => {
                    self.mode = SessionMode::Work;
                    self.time_remaining = std::time::Duration::from_secs(25 * 60);
                }
            }
        }
    }

    pub fn toggle_timer(&mut self) {
        self.is_running = !self.is_running;
    }

    pub fn next_task(&mut self) {
        let i = match self.task_state.selected() {
            Some(i) => {
                if i >= self.tasks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.task_state.select(Some(i));
    }

    pub fn previous_task(&mut self) {
        let i = match self.task_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.tasks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.task_state.select(Some(i));
    }

    pub fn toggle_task(&mut self) {
        if let Some(i) = self.task_state.selected() {
            self.tasks[i].is_done = !self.tasks[i].is_done;
        }
    }

    pub fn get_commands(&self) -> Vec<Command> {
        vec![
            Command { key: "Space", desc: "Toggle Timer" },
            Command { key: "j/k",   desc: "Navigate Tasks" },
            Command { key: "Enter", desc: "Toggle Done" },
            Command { key: "?",     desc: "Toggle Help" },
            Command { key: "q",     desc: "Quit" },
        ]
    }
}
