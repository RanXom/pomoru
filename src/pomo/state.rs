use directories::BaseDirs;
use notify_rust::Notification;
use ratatui::{style::Color, widgets::ListState};
use serde::{Deserialize, Serialize};
use std::{fs, time::Duration};

#[derive(Clone, Copy)]
pub struct Theme {
    pub primary: Color,
    pub overlay0: Color,
    pub surface0: Color,
    pub text: Color,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NoctaliaColors {
    m_primary: String,
    m_on_surface_variant: String,
    m_surface_variant: String,
    m_on_surface: String,
}

fn parse_hex_color(hex: &str) -> Option<Color> {
    if hex.len() == 7 && hex.starts_with('#') {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some(Color::Rgb(r, g, b))
    } else {
        None
    }
}

impl Default for Theme {
    fn default() -> Self {
        let mut theme = Self {
            primary: Color::Rgb(180, 190, 254),
            overlay0: Color::Rgb(108, 112, 134),
            surface0: Color::Rgb(49, 50, 68),
            text: Color::Rgb(205, 214, 244),
        };

        if let Some(base_dirs) = BaseDirs::new() {
            let path = base_dirs.home_dir().join(".config/noctalia/colors.json");
            if let Ok(content) = fs::read_to_string(&path)
                && let Ok(colors) = serde_json::from_str::<NoctaliaColors>(&content)
            {
                if let Some(c) = parse_hex_color(&colors.m_primary) {
                    theme.primary = c;
                }
                if let Some(c) = parse_hex_color(&colors.m_on_surface_variant) {
                    theme.overlay0 = c;
                }
                if let Some(c) = parse_hex_color(&colors.m_surface_variant) {
                    theme.surface0 = c;
                }
                if let Some(c) = parse_hex_color(&colors.m_on_surface) {
                    theme.text = c;
                }
            }
        }
        theme
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum SessionMode {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(PartialEq, Clone, Copy)]
pub enum AppScreen {
    Timer,
    Tasks,
}

#[derive(PartialEq, Clone, Copy)]
pub enum InputMode {
    Normal,
    Insert,
    Edit,
    TimerEdit,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub work_time_mins: u64,
    pub short_break_mins: u64,
    pub long_break_mins: u64,
    pub tasks: Vec<Task>,

    #[serde(default = "default_auto_switch")]
    pub auto_switch_sessions: bool,
}

fn default_auto_switch() -> bool {
    true
}

#[derive(Serialize)]
pub struct CurrentStatus {
    pub text: String,
    pub tooltip: String,
    pub class: String,
}

#[derive(Serialize, Deserialize, Clone)]
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
    pub auto_switch_sessions: bool,
    pub break_count: u32,
    pub tasks: Vec<Task>,
    pub task_state: ListState,
    pub input_buffer: String,
    pub should_quit: bool,
    pub theme: Theme,
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
            auto_switch_sessions: true,
            break_count: 0,
            tasks: Vec::new(),
            task_state: ListState::default(),
            input_buffer: String::new(),
            should_quit: false,
            theme: Theme::default(),
        }
    }

    pub fn tick(&mut self) {
        if self.is_running && self.time_remaining.as_secs() > 0 {
            self.time_remaining -= Duration::from_secs(1);
        } else if self.is_running && self.time_remaining.as_secs() == 0 {
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

            // Use the remaining duration/break count as a seed for simple 'random' selection
            let idx = (self.break_count as usize) % 4;

            let (title, msg) = match self.mode {
                SessionMode::Work => ("Focus Block Complete", focus_msg[idx]),
                _ => ("Break Over", break_msg[idx]),
            };

            self.send_notification(title, msg);

            if self.auto_switch_sessions {
                self.transition_next_session();
                self.is_running = true;
            } else {
                self.is_running = false;
                self.reset_timer_to_mode();
            }
        }
    }

    fn transition_next_session(&mut self) {
        match self.mode {
            SessionMode::Work => {
                self.break_count += 1;
                if self.break_count.is_multiple_of(3) {
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
