use std::fs;
use std::io::Write;

use crate::config::{CACHE_DIR, CONFIG_DIR, LOG_FILE, PACKAGES_DIR};

pub fn init_directories() {
    fs::create_dir_all(PACKAGES_DIR).ok();
    fs::create_dir_all(CONFIG_DIR).ok();
    fs::create_dir_all(CACHE_DIR).ok();
    fs::create_dir_all("/var/log").ok();
}

pub fn log_transaction(action: &str, packages: &[&str]) {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let entry = format!("[{}] {}: {}\n", timestamp, action, packages.join(" "));
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .and_then(|mut file| file.write_all(entry.as_bytes()))
        .ok();
}
