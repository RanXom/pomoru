use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SessionMode {
    Work,
    ShortBreak,
    LongBreak,
}

impl SessionMode {
    pub fn label(self) -> &'static str {
        match self {
            SessionMode::Work => "Work",
            SessionMode::ShortBreak => "Short Break",
            SessionMode::LongBreak => "Long Break",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            SessionMode::Work => "work",
            SessionMode::ShortBreak => "short-break",
            SessionMode::LongBreak => "long-break",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            SessionMode::Work => "󰄉",
            SessionMode::ShortBreak => "󰾅",
            SessionMode::LongBreak => "󰒲",
        }
    }

    pub fn next(self) -> SessionMode {
        match self {
            SessionMode::Work => SessionMode::ShortBreak,
            SessionMode::ShortBreak => SessionMode::LongBreak,
            SessionMode::LongBreak => SessionMode::Work,
        }
    }
}
