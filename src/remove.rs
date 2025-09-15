use crate::config::PACKAGES_DIR;
use crate::utils::log_transaction;
use std::fs;
use std::process::Command;
pub fn remove_packages(packages: &[&str], autoremove: bool) {
    for package in packages {
        println!("--> Removing package: {}", package);
        let obsiext_path = format!("{}/{}.obsiext", PACKAGES_DIR, package);
        Command::new("umount")
            .arg(format!("/run/obsidianos-extensions/{}", package))
            .status()
            .expect("Failed to unmount extension");
        Command::new("obsidianctl")
            .args(&["ext", "rm", package])
            .status()
            .expect("Failed to remove extension");
        fs::remove_file(&obsiext_path).ok();
    }

    Command::new("mount")
        .arg("-a")
        .status()
        .expect("Failed to remount");
    if autoremove {
        autoremove_packages();
    }

    log_transaction("REMOVE", packages);
    println!("--> {} packages removed", packages.len());
}

pub fn autoremove_packages() {
    println!("--> Removing unused dependencies...");
    Command::new("pacman")
        .env("LD_PRELOAD", "")
        .args(&["-Qtdq"])
        .output()
        .map(|output| {
            let orphans = String::from_utf8_lossy(&output.stdout);
            if !orphans.trim().is_empty() {
                for orphan in orphans.lines() {
                    remove_packages(&[orphan.trim()], false);
                }
            } else {
                println!("--> No unused dependencies found");
            }
        })
        .ok();
}
