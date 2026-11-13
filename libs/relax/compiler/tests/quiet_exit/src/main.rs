fn main() {
    if std::env::args().nth(1).as_deref() == Some("success") {
        eprintln!("expected success");
    } else {
        eprintln!("expected refusal");
        std::process::exit(2);
    }
}
