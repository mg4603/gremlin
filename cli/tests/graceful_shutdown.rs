mod common;

use std::process::Stdio;
use std::time::Duration;

use common::assertions::assert_success;
use common::fixtures::wordlist;
use common::process::spawn_cli;

#[test]
#[cfg(unix)]
fn scanner_handles_sigint_gracefully() {
    let child = spawn_cli(
        &[
            "scan",
            "--url",
            "https://example.com/FUZZ",
            "--wordlist",
            wordlist().to_str().unwrap(),
        ],
        Stdio::null(),
    );

    std::thread::sleep(Duration::from_secs(1));

    use nix::sys::signal::{Signal, kill};
    use nix::unistd::Pid;
    kill(Pid::from_raw(child.id() as i32), Signal::SIGINT).unwrap();

    let output = child
        .wait_with_output()
        .expect("failed to wait for process");
    assert_success(&output)
}
