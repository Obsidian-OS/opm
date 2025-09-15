use crate::package_utils::{create_squashfs, extract_package, install_extension, is_installed};
use crate::utils::log_transaction;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
pub fn install_packages(packages: &[&str], reinstall: bool, download_only: bool) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let download_dir = temp_dir.path().join("download");
    fs::create_dir_all(&download_dir).expect("Failed to create download directory");
    for package in packages {
        if !reinstall && is_installed(package) {
            println!("Package {} is already installed", package);
            continue;
        }

        println!("--> Processing package: {}", package);
        let mut pacman_args = vec![
            "-Sw",
            "--noconfirm",
            "--cachedir",
            download_dir.to_str().unwrap(),
        ];
        pacman_args.push(package);
        Command::new("pacman")
            .env("LD_PRELOAD", "")
            .args(&pacman_args)
            .status()
            .expect("Failed to download package");
    }

    if download_only {
        println!("Download completed for {} packages", packages.len());
        return;
    }

    let extract_dir = temp_dir.path().join("extract");
    fs::create_dir_all(&extract_dir).expect("Failed to create extract directory");
    let pkg_files: Vec<PathBuf> = fs::read_dir(&download_dir)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let filename = path.file_name()?.to_str()?;
            if filename.ends_with(".pkg.tar.xz") || filename.ends_with(".pkg.tar.zst") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    println!("--> Extracting {} packages...", pkg_files.len());
    for pkg_file in pkg_files {
        extract_package(&pkg_file, &extract_dir);
    }

    for package in packages {
        let squashfs_path = create_squashfs(package, &extract_dir);
        install_extension(package, &squashfs_path);
    }

    log_transaction("INSTALL", packages);
    println!("--> {} packages installed successfully", packages.len());
}
