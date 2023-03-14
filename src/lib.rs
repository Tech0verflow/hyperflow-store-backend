mod lib{
    pub mod apt;
}

use std::env;
use crate::lib::apt::Apt::{Install, Remove, Search, Update, Upgrade};
use crate::lib::apt::run_apt_command;

fn main() {
    let args: Vec<String> = env::args().collect();
    //Temporary Name: HFS
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
                // Upgrade system
                run_apt_command(Upgrade, "").expect("TODO: panic message");
            }
            "updates" => {
                // Update Ubuntu repositories
                run_apt_command(Update, "").expect("TODO: panic message");
            }
            "install" => {
                // Install package using apt-get
                if args.len() < 3 {
                    eprintln!("Error: No package name provided");
                } else {
                    let package_name = &args[2];
                    run_apt_command(Install, package_name).expect("TODO: panic message");
                }
            }
            "search" => {
                // search package using apt-get
                if args.len() < 3 {
                    eprintln!("Error: No package name provided");
                } else {
                    let package_name = &args[2];
                    run_apt_command(Search, package_name).expect("TODO: panic message");
                }
            }
            "remove" => {
                //remove package using apt-get
                if args.len() < 3 {
                    eprintln!("Error: No package name provided")
                } else {
                    let package_name = &args[2];
                    run_apt_command(Remove, package_name).expect("TODO: panic message");

                }
            }
                    _ => {
                // Display error message for unknown command
                eprintln!("Unknown command '{}'", args[1]);
            }
        }
    }
}
