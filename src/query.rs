use crate::config::{LOG_FILE, PACKAGES_DIR};
use crate::package_utils::is_installed;
use std::fs;
use std::process::Command;
pub fn search_packages(query: &str) {
    println!("--> Searching for: {}", query);
    Command::new("pacman")
        .env("LD_PRELOAD", "")
        .args(&["-Ss", query])
        .status()
        .expect("Failed to search packages");
}

pub fn list_packages(installed: bool, upgradable: bool) {
    if installed {
        println!("--> Installed packages:");
        Command::new("pacman")
            .env("LD_PRELOAD", "")
            .args(&["-Q"])
            .status()
            .expect("Failed to list installed packages");
    } else if upgradable {
        println!("--> Upgradable packages:");
        Command::new("pacman")
            .env("LD_PRELOAD", "")
            .args(&["-Qu"])
            .status()
            .expect("Failed to list upgradable packages");
    } else {
        println!("--> Available packages:");
        Command::new("pacman")
            .env("LD_PRELOAD", "")
            .args(&["-Sl"])
            .status()
            .expect("Failed to list packages");
    }
}

pub fn show_package_info(package: &str) {
    println!("--> Package information for: {}", package);
    Command::new("pacman")
        .env("LD_PRELOAD", "")
        .args(&["-Si", package])
        .status()
        .expect("Failed to show package info");

    if is_installed(package) {
        println!("\n--> Installation status: INSTALLED");
        let obsiext_path = format!("{}/{}.obsiext", PACKAGES_DIR, package);
        if let Ok(metadata) = fs::metadata(&obsiext_path) {
            println!("--> Extension size: {} bytes", metadata.len());
        }
    } else {
        println!("\n--> Installation status: NOT INSTALLED");
    }
}

pub fn show_dependencies(package: &str) {
    println!("--> Dependencies for: {}", package);
    Command::new("pacman")
        .env("LD_PRELOAD", "")
        .args(&["-Si", package])
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.trim().starts_with("Depends On") {
                    println!("{}", line);
                }
            }
        })
        .ok();
}

pub fn list_dependency_packages() {
    println!("--> Installed as dependencies:");
    Command::new("pacman")
        .env("LD_PRELOAD", "")
        .args(&["-Qd"])
        .status()
        .expect("Failed to list dependency packages");
}

pub fn show_history(limit: usize) {
    use std::io::{BufRead, BufReader};

    println!("--> Transaction history (last {} entries):", limit);
    if let Ok(file) = fs::File::open(LOG_FILE) {
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
        for line in lines.iter().rev().take(limit) {
            println!("{}", line);
        }
    } else {
        println!("No transaction history found");
    }
}
