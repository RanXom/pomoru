pub mod state;
pub mod ui;

use crate::pomo::state::{Pomo, AppScreen, InputMode, Task};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;
use std::time::Duration;

impl Pomo {
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
                }
 
                // Tighten poll to 16ms (~60fps feel) for input responsiveness
                event_res = tokio::task::spawn_blocking(|| event::poll(Duration::from_millis(16))) => {
                    if let Ok(Ok(true)) = event_res {
                        if let Ok(Event::Key(key)) = event::read() {
                            self.handle_key(key);
                        }
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match (self.screen, key.code) {
                (_, KeyCode::Char('q')) => self.should_quit = true,
                (AppScreen::Timer, KeyCode::Char('t')) => self.screen = AppScreen::Tasks,
                (AppScreen::Timer, KeyCode::Char(' ')) => self.is_running = !self.is_running,
                (AppScreen::Timer, KeyCode::Char('r')) => self.time_remaining = self.work_time,
                (AppScreen::Tasks, KeyCode::Char('t')) | (AppScreen::Tasks, KeyCode::Esc) => self.screen = AppScreen::Timer,
                (AppScreen::Tasks, KeyCode::Char('i')) => { self.input_mode = InputMode::Insert; self.input_buffer.clear(); }
                (AppScreen::Tasks, KeyCode::Char('e')) => self.enter_edit_mode(),
                (AppScreen::Tasks, KeyCode::Char('d')) => self.delete_task(),
                (AppScreen::Tasks, KeyCode::Char('j')) | (AppScreen::Tasks, KeyCode::Down) => self.next_task(),
                (AppScreen::Tasks, KeyCode::Char('k')) | (AppScreen::Tasks, KeyCode::Up) => self.previous_task(),
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
                        InputMode::Insert => self.tasks.push(Task { title: self.input_buffer.clone(), is_done: false }),
                        InputMode::Edit => if let Some(i) = self.task_state.selected() { self.tasks[i].title = self.input_buffer.clone(); }
                        _ => {}
                    }
                }
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Esc => self.input_mode = InputMode::Normal,
            KeyCode::Backspace => { self.input_buffer.pop(); }
            KeyCode::Char(c) => { self.input_buffer.push(c); }
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
            if self.tasks.is_empty() { self.task_state.select(None); }
        }
    }

    fn toggle_task(&mut self) {
        if let Some(i) = self.task_state.selected() {
            self.tasks[i].is_done = !self.tasks[i].is_done;
        }
    }

    fn next_task(&mut self) {
        let i = match self.task_state.selected() {
            Some(i) => if i >= self.tasks.len() - 1 { 0 } else { i + 1 },
            None => 0,
        };
        self.task_state.select(Some(i));
    }

    fn previous_task(&mut self) {
        let i = match self.task_state.selected() {
            Some(i) => if i == 0 { self.tasks.len() - 1 } else { i - 1 },
            None => 0,
        };
        self.task_state.select(Some(i));
    }
}
