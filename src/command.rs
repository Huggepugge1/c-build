pub fn output(command: &str) -> std::io::Result<std::process::ExitStatus> {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .status()
}

pub fn spawn(command: &str) -> std::io::Result<std::process::Child> {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
}
