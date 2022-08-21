use std::process::Command;
use std::process::Stdio;

fn main() {
    let mut command = Command::new("sh")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Command cannot spawn");

    let _eproc = command.wait().expect("failed to wait for the process");
}
