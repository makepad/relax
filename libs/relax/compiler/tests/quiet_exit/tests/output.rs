use std::process::Command;

#[test]
fn refusal_has_only_the_programs_diagnostic() {
    let output = Command::new(env!("CARGO_BIN_EXE_quiet-exit")).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"expected refusal\n");
}

#[test]
fn success_preserves_the_programs_diagnostic() {
    let output = Command::new(env!("CARGO_BIN_EXE_quiet-exit")).arg("success").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"");
    assert_eq!(output.stderr, b"expected success\n");
}
