use ratatui::widgets::ListState;
use std::time::Duration;
use serde::{ Serialize, Deserialize };
use notify_rust::Notification;

#[derive(PartialEq, Clone, Copy)]
pub enum SessionMode { Work, ShortBreak, LongBreak }

#[derive(PartialEq, Clone, Copy)]
pub enum AppScreen { Timer, Tasks }

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode { Normal, Insert, Edit, TimerEdit }

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub work_time_mins: u64,
    pub short_break_mins: u64,
    pub long_break_mins: u64,
    pub tasks: Vec<Task>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub title: String,
    pub is_done: bool
}

pub struct Pomo {
    pub screen: AppScreen,
    pub mode: SessionMode,
    pub input_mode: InputMode,
    pub work_time: Duration,
    pub short_break_time: Duration,
    pub long_break_time: Duration,
    pub time_remaining: Duration,
    pub total_duration: Duration,
    pub is_running: bool,
    pub break_count: u32,
    pub tasks: Vec<Task>,
    pub task_state: ListState,
    pub input_buffer: String,
    pub should_quit: bool
}

impl Pomo {
    pub fn new() -> Self {
        let work = Duration::from_secs(25 * 60);
        Self {
            screen: AppScreen::Timer,
            mode: SessionMode::Work,
            input_mode: InputMode::Normal,
            work_time: work,
            short_break_time: Duration::from_secs(5 * 60),
            long_break_time: Duration::from_secs(15 * 60),
            time_remaining: work,
            total_duration: work,
            is_running: false,
            break_count: 0,
            tasks: Vec::new(),
            task_state: ListState::default(),
            input_buffer: String::new(),
            should_quit: false
        }
    }

    pub fn tick(&mut self) {
        if self.is_running && self.time_remaining.as_secs() > 0 {
            self.time_remaining -= Duration::from_secs(1);
        } else if self.is_running && self.time_remaining.as_secs() == 0 {
            let (title, msg) = match self.mode {
                SessionMode::Work => (
                    "Focus Block Complete",
                    "I'm tired, boss."
                ),
                _ => (
                    "Break Over",
                    "Ah shit, here we go again."
                ),
            };

            self.send_notification(title, msg);
            self.transition_next_session();
            self.is_running = true;
        }
    }

    fn transition_next_session(&mut self) {
        match self.mode {
            SessionMode::Work => {
                self.break_count += 1;
                if self.break_count % 3 == 0 {
                    self.mode = SessionMode::LongBreak;
                    self.time_remaining = self.long_break_time;
                    self.total_duration = self.long_break_time;
                } else {
                    self.mode = SessionMode::ShortBreak;
                    self.time_remaining = self.short_break_time;
                    self.total_duration = self.short_break_time;
                }
            }
            _ => {
                self.mode = SessionMode::Work;
                self.time_remaining = self.work_time;
                self.total_duration = self.work_time;
            }
        }
    }

    pub fn reset_timer_to_mode(&mut self) {
        self.time_remaining = match self.mode {
            SessionMode::Work => self.work_time,
            SessionMode::ShortBreak => self.short_break_time,
            SessionMode::LongBreak => self.long_break_time,
        };
        self.total_duration = self.time_remaining;
    }

    pub fn send_notification(&self, title: &str, message: &str) {
        let _ = Notification::new()
            .summary(title)
            .body(message)
            .appname("pomoru")
            .timeout(5000)
            .show();
    }
}
