use std::process::Command;

pub enum Apt {
    Upgrade,
    Update,
    Install,
    Search,
    Remove,
}

pub fn run_apt_command(cmd: Apt, package: &str) -> Result<(), String> {
    let (args, cmd_name) = match cmd {
        Apt::Upgrade => (vec!["upgrade", "-y"], "apt", "sudo"),
        Apt::Update => (vec!["update"], "apt", "sudo"),
        Apt::Install => (vec!["install", "-y", package], "apt", "sudo"),
        Apt::Search => (vec!["search", package], "apt", "sudo"),
        Apt::Remove => (vec!["remove", "-y", package], "apt", "sudo")
    };

    let output = Command::new(cmd_name)
        .args(args)
        .output()
        .map_err(|err| format!("Failed to execute command: {}", err))?;

    if !output.status.success() {
        return Err(format!(
            "Command exited with error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("{}", String::from_utf8_lossy(&output.stdout));

    Ok(())
}
