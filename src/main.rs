mod cli;
mod config;
mod dispatcher;
mod install;
mod maintenance;
mod package_utils;
mod query;
mod remove;
mod utils;
use clap::ArgMatches;
use cli::get_matches;
use dispatcher::dispatch;
use std::env;
use utils::init_directories;

fn main() {
    unsafe {
        // note: this is a way to bypass the bug of ObsidianOS Overlays lmao, idk why pacman and chromium hate it..
        env::set_var("LD_PRELOAD", "");
    }

    let matches: ArgMatches = get_matches();
    init_directories();
    dispatch(&matches);
}
