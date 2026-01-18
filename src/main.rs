mod pomo;
use pomo::state::Pomo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Pomo::new();
    app.run()?;
    Ok(())
}
