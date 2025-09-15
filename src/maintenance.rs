use crate::config::CACHE_DIR;
use std::fs;
use std::process::Command;
pub fn update_database() {
    println!("--> Updating package database...");
    Command::new("pacman")
        .env("LD_PRELOAD", "")
        .args(&["-Sy"])
        .status()
        .expect("Failed to update database");
    println!("--> Database updated successfully");
}

pub fn clean_cache(all: bool) {
    if all {
        println!("--> Cleaning all cache...");
        fs::remove_dir_all(CACHE_DIR).ok();
        fs::create_dir_all(CACHE_DIR).ok();
        Command::new("pacman")
            .env("LD_PRELOAD", "")
            .args(&["-Scc", "--noconfirm"])
            .status()
            .expect("Failed to clean pacman cache");
    } else {
        println!("--> Cleaning package cache...");
        Command::new("pacman")
            .env("LD_PRELOAD", "")
            .args(&["-Sc", "--noconfirm"])
            .status()
            .expect("Failed to clean cache");
    }
    println!("--> Cache cleaned");
}
