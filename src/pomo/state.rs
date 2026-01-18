use ratatui::widgets::ListState;
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
pub enum SessionMode { Work, ShortBreak, LongBreak }

#[derive(PartialEq, Clone, Copy)]
pub enum AppScreen { Timer, Tasks }

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode { Normal, Insert, Edit }

pub struct Task {
    pub title: String,
    pub is_done: bool,
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
            self.is_running = false;
            self.transition_next_session();
        }
    }

    fn transition_next_session(&mut self) {
        match self.mode {
            SessionMode::Work => {
                self.break_count += 1;
                if self.break_count % 4 == 0 {
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

    pub fn get_progress(&self) -> f64 {
        if self.total_duration.as_secs() == 0 { return 0.0; }
        let elapsed = self.total_duration.as_secs_f64() - self.time_remaining.as_secs_f64();
        (elapsed / self.total_duration.as_secs_f64()).clamp(0.0, 1.0)
    }
}
