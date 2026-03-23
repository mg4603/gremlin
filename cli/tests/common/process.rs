use std::process::{Child, Command, Output, Stdio};

#[allow(dead_code)]
pub fn spawn_cli(args: &[&str], stdout: Stdio) -> Child {
    Command::new(env!("CARGO_BIN_EXE_cli"))
        .args(args)
        .stdout(stdout)
        .spawn()
        .expect("failed to spawn cli")
}

#[allow(dead_code)]
pub fn run_cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cli"))
        .args(args)
        .output()
        .expect("failed to run cli")
}
