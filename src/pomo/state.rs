use directories::BaseDirs;
use pomoru_core::{
    task::TaskList,
    timer::TimerState,
};
use ratatui::{style::Color, widgets::ListState};
use serde::Deserialize;
use std::fs;

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

/// Thin TUI wrapper around core types.
pub struct Pomo {
    pub timer: TimerState,
    pub tasks: TaskList,
    pub task_state: ListState,
    pub screen: AppScreen,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub should_quit: bool,
    pub theme: Theme,
}

impl Pomo {
    pub fn new() -> Self {
        Self {
            timer: TimerState::new(),
            tasks: TaskList::new(),
            task_state: ListState::default(),
            screen: AppScreen::Timer,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            should_quit: false,
            theme: Theme::default(),
        }
    }

    pub fn load() -> Self {
        let config = pomoru_core::config::load();
        let mut app = Pomo::new();
        app.timer.work_time_secs = config.work_time_mins * 60;
        app.timer.short_break_secs = config.short_break_mins * 60;
        app.timer.long_break_secs = config.long_break_mins * 60;
        app.timer.auto_switch_sessions = config.auto_switch_sessions;
        app.timer.reset_timer_to_mode();
        app.tasks = TaskList::from_tasks(config.tasks);
        app
    }
}

// Re-export core types used by other TUI modules.
pub use pomoru_core::session::SessionMode;
