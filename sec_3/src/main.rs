use std::process::Command;
use std::process::Stdio;

use nix::sched;

fn main() {
    // create isolate namespace
    sched::unshare(
        sched::CloneFlags::CLONE_NEWUTS
            | sched::CloneFlags::CLONE_NEWPID
            | sched::CloneFlags::CLONE_NEWUSER,
    )
    .expect("cannot create unshare");

    let mut command = Command::new("sh")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        // .unshare(&namespaces)
        .spawn()
        .expect("Command cannot spawn");
    let _eproc = command.wait().expect("failed to wait for the process");
}
