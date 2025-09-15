use crate::config::PACKAGES_DIR;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
pub fn extract_package(pkg_file: &Path, extract_dir: &Path) {
    Command::new("tar")
        .env("LD_PRELOAD", "")
        .args(&[
            "-xf",
            pkg_file.to_str().unwrap(),
            "-C",
            extract_dir.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to extract package");
}

pub fn create_squashfs(package: &str, extract_dir: &Path) -> PathBuf {
    fs::create_dir_all(PACKAGES_DIR).expect("Failed to create packages directory");
    let squashfs_path = PathBuf::from(PACKAGES_DIR).join(format!("{}.obsiext", package));
    Command::new("mksquashfs")
        .env("LD_PRELOAD", "")
        .args(&[
            extract_dir.to_str().unwrap(),
            squashfs_path.to_str().unwrap(),
            "-comp",
            "zstd",
            "-b",
            "1M",
            "-quiet",
        ])
        .status()
        .expect("Failed to create squashfs");
    squashfs_path
}

pub fn install_extension(_package: &str, squashfs_path: &Path) {
    Command::new("obsidianctl")
        .env("LD_PRELOAD", "")
        .args(&["ext", "add", squashfs_path.to_str().unwrap()])
        .status()
        .expect("Failed to add extension");

    Command::new("mount")
        .env("LD_PRELOAD", "")
        .arg("-a")
        .status()
        .expect("Failed to remount");
}

pub fn is_installed(package: &str) -> bool {
    let pacman_installed = Command::new("pacman")
        .args(&["-Q", package])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    let obsiext_path = format!("{}/{}.obsiext", PACKAGES_DIR, package);
    let obsiext_installed = Path::new(&obsiext_path).exists();
    pacman_installed || obsiext_installed
}
