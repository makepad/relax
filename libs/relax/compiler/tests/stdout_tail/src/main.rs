use std::io::Write;

fn main() {
    std::io::stdout().lock().write_all(b"tail").unwrap();
}
