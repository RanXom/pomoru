use crate::task::Task;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for Config {
    fn default() -> Self {
        Self {
            work_time_mins: 25,
            short_break_mins: 5,
            long_break_mins: 15,
            tasks: Vec::new(),
            auto_switch_sessions: true,
        }
    }
}

pub fn config_dir() -> Result<std::path::PathBuf, String> {
    ProjectDirs::from("", "", "pomoru")
        .map(|dirs| dirs.config_dir().to_path_buf())
        .ok_or_else(|| "Could not find config directory".to_string())
}

pub fn load() -> Config {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "pomoru") {
        let config_path = proj_dirs.config_dir().join("config.toml");
        if let Ok(content) = fs::read_to_string(config_path)
            && let Ok(config) = toml::from_str::<Config>(&content)
        {
            return config;
        }
    }
    Config::default()
}

pub fn save(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let toml_str = toml::to_string_pretty(config)?;
    let dir = config_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("config.toml"), toml_str)?;
    Ok(())
}
