use crate::install::install_packages;
use crate::maintenance::{clean_cache, update_database};
use crate::query::{
    list_dependency_packages, list_packages, search_packages, show_dependencies, show_history, show_package_info,
};
use crate::remove::autoremove_packages;
use crate::remove::remove_packages;
use clap::ArgMatches;
pub fn dispatch(matches: &ArgMatches) {
    if matches.get_flag("sync") {
        if matches.get_flag("update") {
            update_database();
            return;
        }
        let packages: Vec<String> = matches
            .get_many::<String>("packages")
            .unwrap_or_default()
            .cloned()
            .collect();
        if matches.get_flag("search") {
            if packages.len() == 1 {
                search_packages(&packages[0]);
            } else if packages.is_empty() {
                eprintln!("Error: -Ss requires a search term");
                std::process::exit(1);
            } else {
                eprintln!("Error: -Ss requires exactly one search term");
                std::process::exit(1);
            }
            return;
        }

        if matches.get_flag("info") {
            if packages.len() == 1 {
                show_package_info(&packages[0]);
            } else if packages.is_empty() {
                eprintln!("Error: -Si requires a package name");
                std::process::exit(1);
            } else {
                eprintln!("Error: -Si requires exactly one package name");
                std::process::exit(1);
            }
            return;
        }

        if matches.get_flag("deps") {
            if packages.len() == 1 {
                show_dependencies(&packages[0]);
            } else if packages.is_empty() {
                eprintln!("Error: -Sd requires a package name");
                std::process::exit(1);
            }
            return;
        }

        if !packages.is_empty() {
            let packages_str: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
            install_packages(&packages_str, false, false);
            return;
        } else {
            eprintln!("Error: -S requires package names");
            std::process::exit(1);
        }
    }

    if matches.get_flag("remove") {
        let packages: Vec<String> = matches
            .get_many::<String>("packages")
            .unwrap_or_default()
            .cloned()
            .collect();
        if !packages.is_empty() {
            let packages_str: Vec<&str> = packages.iter().map(|s| s.as_str()).collect();
            remove_packages(&packages_str, false);
            return;
        } else {
            eprintln!("Error: -R requires package names");
            std::process::exit(1);
        }
    }

    if matches.get_flag("query") {
        if matches.get_flag("info") {
            let packages: Vec<String> = matches
                .get_many::<String>("packages")
                .unwrap_or_default()
                .cloned()
                .collect();
            if packages.len() == 1 {
                show_package_info(&packages[0]);
            } else if packages.is_empty() {
                eprintln!("Error: -Qi requires a package name");
                std::process::exit(1);
            } else {
                eprintln!("Error: -Qi requires exactly one package name");
                std::process::exit(1);
            }
            return;
        }
        if matches.get_flag("deps") {
            let packages: Vec<String> = matches
                .get_many::<String>("packages")
                .unwrap_or_default()
                .cloned()
                .collect();
            if packages.is_empty() {
                list_dependency_packages();
            } else {
                eprintln!("Error: -Qd does not take package names");
                std::process::exit(1);
            }
            return;
        }
        if matches.get_flag("upgradable") {
            list_packages(false, true);
            return;
        }
        list_packages(true, false);
        return;
    }

    match matches.subcommand() {
        Some(("install", sub_m)) => {
            let packages_str: Vec<&str> = sub_m
                .get_many::<String>("packages")
                .unwrap()
                .map(|s| s.as_str())
                .collect();
            let reinstall = sub_m.get_flag("reinstall");
            let download_only = sub_m.get_flag("download-only");
            install_packages(&packages_str, reinstall, download_only);
        }
        Some(("remove", sub_m)) => {
            let packages_str: Vec<&str> = sub_m
                .get_many::<String>("packages")
                .unwrap()
                .map(|s| s.as_str())
                .collect();
            let autoremove = sub_m.get_flag("autoremove");
            remove_packages(&packages_str, autoremove);
        }
        Some(("update", _)) => {
            update_database();
        }
        Some(("search", sub_m)) => {
            let query = sub_m.get_one::<String>("query").unwrap();
            search_packages(query);
        }
        Some(("list", sub_m)) => {
            let installed = sub_m.get_flag("installed");
            let upgradable = sub_m.get_flag("upgradable");
            list_packages(installed, upgradable);
        }
        Some(("show", sub_m)) => {
            let package = sub_m.get_one::<String>("package").unwrap();
            show_package_info(package);
        }
        Some(("depends", sub_m)) => {
            let package = sub_m.get_one::<String>("package").unwrap();
            show_dependencies(package);
        }
        Some(("clean", sub_m)) => {
            let all = sub_m.get_flag("all");
            clean_cache(all);
        }
        Some(("autoremove", _)) => {
            autoremove_packages();
        }
        Some(("history", sub_m)) => {
            let limit = sub_m
                .get_one::<String>("limit")
                .map(|s| s.parse().unwrap_or(10))
                .unwrap_or(10);
            show_history(limit);
        }
        _ => {
            eprintln!("Usage: opm <command> [options]");
            eprintln!("Try 'opm --help' for more information");
            std::process::exit(1);
        }
    }
}
