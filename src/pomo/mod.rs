pub mod state;
pub mod ui;

use crate::pomo::state::{AppScreen, InputMode, Pomo};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use pomoru_core::{config, notification, status};
use ratatui::prelude::*;
use std::{io, time::Duration};

impl Pomo {
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cfg = config::Config {
            work_time_mins: self.timer.work_time_secs / 60,
            short_break_mins: self.timer.short_break_secs / 60,
            long_break_mins: self.timer.long_break_secs / 60,
            tasks: self.tasks.items().to_vec(),
            auto_switch_sessions: self.timer.auto_switch_sessions,
        };
        config::save(&cfg)
    }

    pub fn export_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        status::export_status(self.timer.mode, self.timer.time_remaining_secs)
    }

    pub fn clear_status(&self) -> Result<(), Box<dyn std::error::Error>> {
        status::clear_status()
    }

    pub async fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut second_tick = tokio::time::interval(Duration::from_secs(1));

        let _ = self.export_status();

        while !self.should_quit {
            terminal.draw(|f| ui::render(f, self))?;

            tokio::select! {
                _ = second_tick.tick() => {
                    if let Some(event) = self.timer.tick() {
                        notification::send_notification(&event.title, &event.message);
                    }
                    let _ = self.export_status();
                }

                // Tighten poll to 16ms (~60fps feel) for input responsiveness
                event_res = tokio::task::spawn_blocking(|| event::poll(Duration::from_millis(16))) => {
                    if let Ok(Ok(true)) = event_res
                        && let Ok(Event::Key(key)) = event::read()
                        && key.kind == event::KeyEventKind::Press
                    {
                        self.handle_key(key);
                        let _ = self.export_status();
                    }
               }
            }

            if self.should_quit {
                let _ = self.save();
                let _ = self.clear_status();
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
                    self.timer.cycle_mode();
                }

                (AppScreen::Timer, KeyCode::Char('e')) => {
                    if !self.timer.is_running {
                        self.input_mode = InputMode::TimerEdit;
                        self.input_buffer =
                            (self.timer.time_remaining_secs / 60).to_string();
                    }
                }

                (AppScreen::Timer, KeyCode::Char('a')) => {
                    self.timer.toggle_auto_switch();
                }
                (AppScreen::Timer, KeyCode::Char('t')) => self.screen = AppScreen::Tasks,
                (AppScreen::Timer, KeyCode::Char(' ')) => self.timer.toggle_running(),
                (AppScreen::Timer, KeyCode::Char('r')) => {
                    self.timer.reset_timer_to_mode();
                }
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
                    self.tasks.select_next();
                    self.sync_task_state();
                }
                (AppScreen::Tasks, KeyCode::Char('k')) | (AppScreen::Tasks, KeyCode::Up) => {
                    self.tasks.select_previous();
                    self.sync_task_state();
                }
                (AppScreen::Tasks, KeyCode::Char('J')) => {
                    self.tasks.move_down();
                    self.sync_task_state();
                }
                (AppScreen::Tasks, KeyCode::Char('K')) => {
                    self.tasks.move_up();
                    self.sync_task_state();
                }
                (AppScreen::Tasks, KeyCode::Enter) => {
                    if let Some(i) = self.tasks.selected() {
                        self.tasks.toggle(i);
                    }
                }
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
                                self.timer.set_current_duration_mins(mins);
                            }
                        }

                        InputMode::Insert => {
                            self.tasks.add(self.input_buffer.clone());
                            self.sync_task_state();
                        }

                        InputMode::Edit => {
                            if let Some(i) = self.tasks.selected() {
                                self.tasks.edit(i, self.input_buffer.clone());
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
        if let Some(i) = self.tasks.selected() {
            self.input_mode = InputMode::Edit;
            self.input_buffer = self.tasks.items()[i].title.clone();
        }
    }

    fn delete_task(&mut self) {
        if let Some(i) = self.tasks.selected() {
            self.tasks.delete(i);
            self.sync_task_state();
        }
    }

    /// Sync the ratatui ListState with the core TaskList selection.
    fn sync_task_state(&mut self) {
        self.task_state.select(self.tasks.selected());
    }
}
