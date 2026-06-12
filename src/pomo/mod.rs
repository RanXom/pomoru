pub mod state;
pub mod ui;

use crate::pomo::{
    state::{AppScreen, Config, CurrentStatus, InputMode, Pomo, SessionMode, Task},
    ui::format_duration,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use directories::ProjectDirs;
use ratatui::prelude::*;
use std::{fs, io, time::Duration};

impl Pomo {
    pub fn current_status(&self) -> CurrentStatus {
        let class = match self.mode {
            SessionMode::ShortBreak => "short-break",
            SessionMode::LongBreak => "long-break",
            SessionMode::Work => "work",
        };

        let tooltip = match self.mode {
            SessionMode::ShortBreak => "Short Break",
            SessionMode::LongBreak => "Long Break",
            SessionMode::Work => "Work",
        };

        CurrentStatus {
            text: format_duration(self.time_remaining),
            tooltip: tooltip.to_string(),
            class: class.to_string(),
        }
    }

    pub fn export_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        let status = self.current_status();

        let json = serde_json::to_string(&status)?;

        let cache_dir = ProjectDirs::from("", "", "pomoru")
            .ok_or("Could not find cache directory")?
            .cache_dir()
            .to_path_buf();

        fs::create_dir_all(&cache_dir)?;
        fs::write(cache_dir.join("status.json"), json)?;

        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config {
            work_time_mins: self.work_time.as_secs() / 60,
            short_break_mins: self.short_break_time.as_secs() / 60,
            long_break_mins: self.long_break_time.as_secs() / 60,
            tasks: self.tasks.clone(),
            auto_switch_sessions: self.auto_switch_sessions,
        };

        let toml = toml::to_string_pretty(&config)?;
        let config_dir = ProjectDirs::from("", "", "pomoru")
            .ok_or("Could not find config directory")?
            .config_dir()
            .to_path_buf();

        fs::create_dir_all(&config_dir)?;
        fs::write(config_dir.join("config.toml"), toml)?;
        Ok(())
    }

    pub fn load() -> Self {
        let mut app = Pomo::new();
        if let Some(proj_dirs) = ProjectDirs::from("", "", "pomoru") {
            let config_path = proj_dirs.config_dir().join("config.toml");
            if let Ok(content) = fs::read_to_string(config_path)
                && let Ok(config) = toml::from_str::<Config>(&content)
            {
                app.work_time = Duration::from_secs(config.work_time_mins * 60);
                app.short_break_time = Duration::from_secs(config.short_break_mins * 60);
                app.long_break_time = Duration::from_secs(config.long_break_mins * 60);
                app.tasks = config.tasks;
                app.auto_switch_sessions = config.auto_switch_sessions;
                app.reset_timer_to_mode();
            }
        }
        app
    }

    pub async fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut second_tick = tokio::time::interval(Duration::from_secs(1));

        while !self.should_quit {
            terminal.draw(|f| ui::render(f, self))?;

            tokio::select! {
                _ = second_tick.tick() => {
                    self.tick();
                    let _ = self.export_status();
                }

                // Tighten poll to 16ms (~60fps feel) for input responsiveness
                event_res = tokio::task::spawn_blocking(|| event::poll(Duration::from_millis(16))) => {
                    if let Ok(Ok(true)) = event_res
                        && let Ok(Event::Key(key)) = event::read()
                        && key.kind == event::KeyEventKind::Press
                    {
                        self.handle_key(key);
                    }
               }
            }

            if self.should_quit {
                let _ = self.save();
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match (self.screen, key.code) {
                (AppScreen::Tasks, KeyCode::Char('q')) => self.screen = AppScreen::Timer,
                (AppScreen::Timer, KeyCode::Char('q')) => self.should_quit = true,

                (AppScreen::Timer, KeyCode::Tab) => {
                    if !self.is_running {
                        self.mode = match self.mode {
                            SessionMode::Work => SessionMode::ShortBreak,
                            SessionMode::ShortBreak => SessionMode::LongBreak,
                            SessionMode::LongBreak => SessionMode::Work,
                        };
                        self.reset_timer_to_mode();
                    }
                }

                (AppScreen::Timer, KeyCode::Char('e')) => {
                    if !self.is_running {
                        self.input_mode = InputMode::TimerEdit;
                        self.input_buffer = (self.time_remaining.as_secs() / 60).to_string();
                    }
                }

                (AppScreen::Timer, KeyCode::Char('a')) => {
                    self.auto_switch_sessions = !self.auto_switch_sessions
                }
                (AppScreen::Timer, KeyCode::Char('t')) => self.screen = AppScreen::Tasks,
                (AppScreen::Timer, KeyCode::Char(' ')) => self.is_running = !self.is_running,
                (AppScreen::Timer, KeyCode::Char('r')) => self.time_remaining = self.work_time,
                (AppScreen::Tasks, KeyCode::Char('t')) | (AppScreen::Tasks, KeyCode::Esc) => {
                    self.screen = AppScreen::Timer
                }
                (AppScreen::Tasks, KeyCode::Char('i')) => {
                    self.input_mode = InputMode::Insert;
                    self.input_buffer.clear();
                }
                (AppScreen::Tasks, KeyCode::Char('e')) => self.enter_edit_mode(),
                (AppScreen::Tasks, KeyCode::Char('d')) => self.delete_task(),
                (AppScreen::Tasks, KeyCode::Char('j')) | (AppScreen::Tasks, KeyCode::Down) => {
                    self.next_task()
                }
                (AppScreen::Tasks, KeyCode::Char('k')) | (AppScreen::Tasks, KeyCode::Up) => {
                    self.previous_task()
                }
                (AppScreen::Tasks, KeyCode::Char('J')) => self.move_task_down(),
                (AppScreen::Tasks, KeyCode::Char('K')) => self.move_task_up(),
                (AppScreen::Tasks, KeyCode::Enter) => self.toggle_task(),
                _ => {}
            },
            _ => self.handle_input_mode(key),
        }
    }

    fn handle_input_mode(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                if !self.input_buffer.is_empty() {
                    match self.input_mode {
                        InputMode::TimerEdit => {
                            if let Ok(mins) = self.input_buffer.parse::<u64>() {
                                let new_dur = Duration::from_secs(mins * 60);
                                match self.mode {
                                    SessionMode::Work => self.work_time = new_dur,
                                    SessionMode::ShortBreak => self.short_break_time = new_dur,
                                    SessionMode::LongBreak => self.long_break_time = new_dur,
                                }
                                self.time_remaining = new_dur;
                                self.total_duration = new_dur;
                            }
                        }

                        InputMode::Insert => self.tasks.push(Task {
                            title: self.input_buffer.clone(),
                            is_done: false,
                        }),

                        InputMode::Edit => {
                            if let Some(i) = self.task_state.selected() {
                                self.tasks[i].title = self.input_buffer.clone();
                            }
                        }

                        _ => {}
                    }
                }
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Esc => self.input_mode = InputMode::Normal,
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
    }

    fn enter_edit_mode(&mut self) {
        if let Some(i) = self.task_state.selected() {
            self.input_mode = InputMode::Edit;
            self.input_buffer = self.tasks[i].title.clone();
        }
    }

    fn delete_task(&mut self) {
        if let Some(i) = self.task_state.selected() {
            self.tasks.remove(i);
            if self.tasks.is_empty() {
                self.task_state.select(None);
            }
        }
    }

    fn toggle_task(&mut self) {
        if let Some(i) = self.task_state.selected() {
            self.tasks[i].is_done = !self.tasks[i].is_done;
        }
    }

    fn next_task(&mut self) {
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

    fn previous_task(&mut self) {
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

    pub(crate) fn move_task_up(&mut self) {
        match self.task_state.selected() {
            Some(0) => (),
            Some(i) => {
                let temp = self.tasks[i].clone();
                let j = (i + self.tasks.len() - 1) % self.tasks.len();
                self.tasks[i] = self.tasks[j].clone();
                self.tasks[j] = temp;
                self.previous_task();
            }
            None => (),
        }
    }

    pub(crate) fn move_task_down(&mut self) {
        match self.task_state.selected() {
            Some(val) if val == self.tasks.len() - 1 => (),
            Some(i) => {
                let temp = self.tasks[i].clone();
                let j = (i + 1) % self.tasks.len();
                self.tasks[i] = self.tasks[j].clone();
                self.tasks[j] = temp;
                self.next_task();
            }
            None => (),
        }
    }
}
