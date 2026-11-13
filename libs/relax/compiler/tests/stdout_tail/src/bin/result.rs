use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    std::io::stdout().lock().write_all(b"result-tail")?;
    Ok(())
}
