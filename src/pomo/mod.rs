pub mod state;
pub mod ui;

use crate::pomo::state::Pomo;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{ io, time::Duration };
use tokio::time::{self, MissedTickBehavior};

impl Pomo {
    pub async fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut interval = time::interval(Duration::from_millis(250));
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        let mut second_tick = time::interval(Duration::from_secs(1));

        while !self.should_quit {
            terminal.draw(|f| ui::render(f, self))?;

            tokio::select! {
                // Handle key events
                _ = interval.tick() => {
                    if event::poll(Duration::from_millis(0))? {
                        if let Event::Key(key) = event::read()? {
                            self.handle_key(key);
                        }
                    }
                }
                _ = second_tick.tick() => {
                    self.tick();
                }
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('?') => self.show_help = !self.show_help,
            KeyCode::Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.show_help = !self.show_help
            }
            _ if !self.show_help => match key.code {
                KeyCode::Char(' ') => self.toggle_timer(),
                KeyCode::Char('j') | KeyCode::Down => self.next_task(),
                KeyCode::Char('k') | KeyCode::Up => self.previous_task(),
                KeyCode::Enter => self.toggle_task(),
                _ => {}
            },
            _ => {}
        }
    }
}
