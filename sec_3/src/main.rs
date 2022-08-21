use unshare::Namespace;
use unshare::Stdio;

fn main() {
    let mut namespaces = Vec::<Namespace>::new();
    namespaces.push(Namespace::Pid);
    namespaces.push(Namespace::Mount);
    // let mut cmd = std::env::args().nth(1);
    let mut command = unshare::Command::new("ls")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        // .unshare(&namespaces)
        .spawn()
        .expect("Command cannot spawn");
    let _eproc = command.wait().expect("failed to wait for the process");
}
