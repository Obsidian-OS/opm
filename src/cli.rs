use clap::{Arg, ArgAction, Command as ClapCommand};
pub fn get_matches() -> clap::ArgMatches {
    ClapCommand::new("opm")
        .override_usage("opm <command> [options] | opm -S <packages> | opm -R <packages> | opm -Q [options] | opm -Ss <query>")
        .arg(Arg::new("sync").short('S').help("Install packages").action(ArgAction::SetTrue))
        .arg(Arg::new("remove").short('R').help("Remove packages").action(ArgAction::SetTrue))
        .arg(Arg::new("query").short('Q').help("Query packages").action(ArgAction::SetTrue))
        .arg(Arg::new("search").short('s').help("Search (use with -S)").action(ArgAction::SetTrue))
        .arg(Arg::new("info").short('i').help("Package info (use with -Q or -S)").action(ArgAction::SetTrue))
        .arg(Arg::new("deps").short('d').help("Show deps (use with -Q or -S)").action(ArgAction::SetTrue))
        .arg(Arg::new("upgradable").short('u').help("Show upgradable (use with -Q)").action(ArgAction::SetTrue))
        .arg(Arg::new("update").short('y').help("Update database (use with -S)").action(ArgAction::SetTrue))
        .arg(Arg::new("packages").num_args(0..).value_name("PACKAGES"))
        .version("1.0.0")
        .about("ObsidianOS Package Manager")
        .subcommand(
            ClapCommand::new("install")
                .about("Install packages")
                .arg(Arg::new("packages").required(true).num_args(1..))
                .arg(Arg::new("reinstall").long("reinstall").help("Force reinstall").action(clap::ArgAction::SetTrue))
                .arg(Arg::new("download-only").long("download-only").short('d').help("Download only").action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            ClapCommand::new("remove")
                .about("Remove packages")
                .arg(Arg::new("packages").required(true).num_args(1..))
                .arg(Arg::new("autoremove").long("autoremove").help("Remove unused deps").action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            ClapCommand::new("update")
                .about("Update package database")
        )
        .subcommand(
            ClapCommand::new("search")
                .about("Search for packages")
                .arg(Arg::new("query").required(true))
        )
        .subcommand(
            ClapCommand::new("list")
                .about("List packages")
                .arg(Arg::new("installed").long("installed").help("Show installed only").action(clap::ArgAction::SetTrue))
                .arg(Arg::new("upgradable").long("upgradable").help("Show upgradable").action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            ClapCommand::new("show")
                .about("Show package information")
                .arg(Arg::new("package").required(true))
        )
        .subcommand(
            ClapCommand::new("depends")
                .about("Show package dependencies")
                .arg(Arg::new("package").required(true))
        )
        .subcommand(
            ClapCommand::new("clean")
                .about("Clean package cache")
                .arg(Arg::new("all").long("all").help("Clean everything").action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            ClapCommand::new("autoremove")
                .about("Remove unused dependencies")
        )
        .subcommand(
            ClapCommand::new("history")
                .about("Show transaction history")
                .arg(Arg::new("limit").long("limit").short('n').value_name("NUM").help("Limit entries"))
        )
        .get_matches()
}
