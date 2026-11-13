use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    std::io::stdout().lock().write_all(b"code-tail").unwrap();
    ExitCode::SUCCESS
}
