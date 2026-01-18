mod pomo;
use pomo::state::Pomo;
use std::panic;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::execute;

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal state before exiting
        let _ = disable_raw_mode();
        let mut stdout = std::io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);

        // Call the original hook to print the error
        original_hook(panic_info);
    }));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic_hook();
    let mut app = Pomo::new();
    app.run().await?;
    Ok(())
}
