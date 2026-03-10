use std::process::{Command, Stdio};
use std::time::Duration;

#[test]
#[cfg(unix)]
fn scanner_handles_sigint_gracefully() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_cli"))
        .arg("scan")
        .arg("--url")
        .arg("https://example.com/FUZZ")
        .arg("--wordlist")
        .arg("tests/data/wordlist.txt")
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to spawn scanner");

    std::thread::sleep(Duration::from_secs(1));

    use nix::sys::signal::{Signal, kill};
    use nix::unistd::Pid;
    kill(Pid::from_raw(child.id() as i32), Signal::SIGINT).unwrap();

    let status = child.wait().expect("failed to wait for process");
    assert!(status.success());
}
