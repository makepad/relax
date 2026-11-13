use std::process::Command;

#[test]
fn unit_main_flushes_stdout_tail() {
    let output = Command::new(env!("CARGO_BIN_EXE_stdout-tail")).output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"tail");
    assert_eq!(output.stderr, b"");
}

#[test]
fn result_main_flushes_stdout_tail() {
    let output = Command::new(env!("CARGO_BIN_EXE_result")).output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"result-tail");
    assert_eq!(output.stderr, b"");
}

#[test]
fn exit_code_main_flushes_stdout_tail() {
    let output = Command::new(env!("CARGO_BIN_EXE_exit_code")).output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"code-tail");
    assert_eq!(output.stderr, b"");
}
