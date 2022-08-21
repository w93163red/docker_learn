use std::env;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let mut args = env::args();
    let cmd = args.nth(1).expect("cannot get the command");
    let mut command = Command::new(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Command cannot spawn");

    let _eproc = command.wait().expect("failed to wait for the process");
}
