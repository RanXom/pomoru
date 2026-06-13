use crate::session::SessionMode;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Event emitted by `TimerState::tick()` when a session completes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerEvent {
    pub title: String,
    pub message: String,
}

// Pure timer state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub mode: SessionMode,
    pub work_time_secs: u64,
    pub short_break_secs: u64,
    pub long_break_secs: u64,
    pub time_remaining_secs: u64,
    pub total_duration_secs: u64,
    pub is_running: bool,
    pub auto_switch_sessions: bool,
    pub break_count: u32,
}

impl Default for TimerState {
    fn default() -> Self {
        let work_secs: u64 = 25 * 60;
        Self {
            mode: SessionMode::Work,
            work_time_secs: work_secs,
            short_break_secs: 5 * 60,
            long_break_secs: 15 * 60,
            time_remaining_secs: work_secs,
            total_duration_secs: work_secs,
            is_running: false,
            auto_switch_sessions: true,
            break_count: 0,
        }
    }
}

impl TimerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) -> Option<TimerEvent> {
        if self.is_running && self.time_remaining_secs > 0 {
            self.time_remaining_secs -= 1;
            None
        } else if self.is_running && self.time_remaining_secs == 0 {
            let focus_msg = [
                "I'm tired, boss...",
                "Congrats! You're him 🗿",
                "Stand up. Touch grass.",
                "Mission Passed! Respect+",
            ];
            let break_msg = [
                "Ah shit, here we go again.",
                "Wake up, Samurai. We have code to burn.",
                "Lock back in.",
                "Ref! Do Something! The break's over!",
            ];

            let idx = (self.break_count as usize) % 4;

            let (title, msg) = match self.mode {
                SessionMode::Work => ("Focus Block Complete", focus_msg[idx]),
                _ => ("Break Over", break_msg[idx]),
            };

            let event = TimerEvent {
                title: title.to_string(),
                message: msg.to_string(),
            };

            if self.auto_switch_sessions {
                self.transition_next_session();
                self.is_running = true;
            } else {
                self.is_running = false;
                self.reset_timer_to_mode();
            }

            Some(event)
        } else {
            None
        }
    }

    fn transition_next_session(&mut self) {
        match self.mode {
            SessionMode::Work => {
                self.break_count += 1;
                if self.break_count.is_multiple_of(3) {
                    self.mode = SessionMode::LongBreak;
                    self.time_remaining_secs = self.long_break_secs;
                    self.total_duration_secs = self.long_break_secs;
                } else {
                    self.mode = SessionMode::ShortBreak;
                    self.time_remaining_secs = self.short_break_secs;
                    self.total_duration_secs = self.short_break_secs;
                }
            }
            _ => {
                self.mode = SessionMode::Work;
                self.time_remaining_secs = self.work_time_secs;
                self.total_duration_secs = self.work_time_secs;
            }
        }
    }

    pub fn reset_timer_to_mode(&mut self) {
        self.time_remaining_secs = match self.mode {
            SessionMode::Work => self.work_time_secs,
            SessionMode::ShortBreak => self.short_break_secs,
            SessionMode::LongBreak => self.long_break_secs,
        };
        self.total_duration_secs = self.time_remaining_secs;
    }

    pub fn set_mode(&mut self, mode: SessionMode) {
        if !self.is_running {
            self.mode = mode;
            self.reset_timer_to_mode();
        }
    }

    pub fn cycle_mode(&mut self) {
        if !self.is_running {
            self.mode = self.mode.next();
            self.reset_timer_to_mode();
        }
    }

    pub fn toggle_running(&mut self) {
        self.is_running = !self.is_running;
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn pause(&mut self) {
        self.is_running = false;
    }

    pub fn reset(&mut self) {
        self.is_running = false;
        self.reset_timer_to_mode();
    }

    pub fn skip_session(&mut self) {
        self.transition_next_session();
        self.is_running = false;
    }

    pub fn toggle_auto_switch(&mut self) {
        self.auto_switch_sessions = !self.auto_switch_sessions;
    }

    pub fn set_current_duration_mins(&mut self, mins: u64) {
        let secs = mins * 60;
        match self.mode {
            SessionMode::Work => self.work_time_secs = secs,
            SessionMode::ShortBreak => self.short_break_secs = secs,
            SessionMode::LongBreak => self.long_break_secs = secs,
        }
        self.time_remaining_secs = secs;
        self.total_duration_secs = secs;
    }

    pub fn time_remaining(&self) -> Duration {
        Duration::from_secs(self.time_remaining_secs)
    }
    pub fn total_duration(&self) -> Duration {
        Duration::from_secs(self.total_duration_secs)
    }

    pub fn progress(&self) -> f64 {
        if self.total_duration_secs == 0 {
            return 0.0;
        }
        1.0 - (self.time_remaining_secs as f64 / self.total_duration_secs as f64)
    }
}

pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
