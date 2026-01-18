pub mod state;
pub mod ui;

use crate::pomo::state::Pomo;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;
use std::time::{Duration, Instant}; // Fixed: Added Instant here

impl Pomo {
    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(250);

        while !self.should_quit {
            terminal.draw(|f| ui::render(f, self))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => self.should_quit = true,
                        KeyCode::Char(' ') => self.toggle_timer(),
                        _ => {}
                    }
                }
            }

            if last_tick.elapsed() >= Duration::from_secs(1) {
                self.tick();
                last_tick = Instant::now();
            }
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }
}
