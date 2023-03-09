use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // Display help information
        println!("HFS Command Line Interface");
        println!("Usage:");
        println!("hfs upgrade\tUpgrades System");
        println!("hfs updates\tUpdates Ubuntu Repositories");
        println!("hfs install [PACKAGE]\tInstalls a package using apt-get");
    } else {
        match args[1].as_str() {
            "upgrade" => {
                // Implement 'upgrade' command
                let output = Command::new("sudo")
                    .arg("apt")
                    .arg("upgrade")
                    .arg("-y")
                    .output()
                    .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "updates" => {
                // Implement 'updates' command
                let output = Command::new("apt")
                    .arg("update")
                    .output()
                    .expect("Failed to execute command");

                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            "install" => {
                // Implement 'install' command
                if args.len() < 3 {
                    eprintln!("Error: No package name provided");
                } else {
                    let package_name = &args[2];
                    let output = Command::new("sudo")
                        .arg("apt")
                        .arg("install")
                        .arg("-y")
                        .arg(package_name)
                        .output()
                        .expect("Failed to execute command");
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            "search" => {
                // Implement 'search' command
                if args.len() < 3 {
                    eprintln!("Error: No package name provided");
                } else {
                    let package_name = &args[2];
                    let output = Command::new("apt")
                        .arg("search")
                        .arg(package_name)
                        .output()
                        .expect("Failed to execute command");
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            _ => {
                // Display error message for unknown command
                eprintln!("Unknown command '{}'", args[1]);
            }
        }
    }
}