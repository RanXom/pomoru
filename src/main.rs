mod pomo;
use pomo::Pomo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Pomo::new();
    app.run()?;
    Ok(())
}
