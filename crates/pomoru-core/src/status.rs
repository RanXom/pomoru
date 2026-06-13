use crate::session::SessionMode;
use crate::timer::format_duration;
use directories::ProjectDirs;
use serde::Serialize;
use std::{fs, time::Duration};

#[derive(Debug, Clone, Serialize)]
pub struct CurrentStatus {
    pub text: String,
    pub tooltip: String,
    pub class: String,
}

pub fn build_status(mode: SessionMode, time_remaining_secs: u64) -> CurrentStatus {
    let duration = Duration::from_secs(time_remaining_secs);
    CurrentStatus {
        text: format!(
            "{} {} {}",
            mode.icon(),
            mode.label(),
            format_duration(duration)
        ),
        tooltip: mode.label().to_string(),
        class: mode.class().to_string(),
    }
}

pub fn export_status(mode: SessionMode, time_remaining_secs: u64) -> Result<(), Box<dyn std::error::Error>> {
    let status = build_status(mode, time_remaining_secs);
    let json = serde_json::to_string(&status)?;

    let cache_dir = ProjectDirs::from("", "", "pomoru")
        .ok_or("Could not find cache directory")?
        .cache_dir()
        .to_path_buf();

    fs::create_dir_all(&cache_dir)?;
    fs::write(cache_dir.join("status.json"), json)?;

    Ok(())
}

pub fn clear_status() -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = ProjectDirs::from("", "", "pomoru")
        .ok_or("Could not find cache directory")?
        .cache_dir()
        .to_path_buf();

    let status_file = cache_dir.join("status.json");
    if status_file.exists() {
        fs::remove_file(status_file)?;
    }

    Ok(())
}