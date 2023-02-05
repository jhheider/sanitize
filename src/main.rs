mod clap;
mod config;
mod execution;
mod logger;

use crate::config::Config;
use crate::execution::Execution;
use colored::Colorize;
use std::process::exit;

#[cfg(not(tarpaulin_include))]
fn main() {
    let setup = clap::setup();
    let matches = setup.get_matches();
    let config = Config::from_matches(&matches);
    if config.is_err() {
        eprintln!("[{}]: {:?}", "error".red().bold(), config.unwrap_err());
        exit(1);
    }
    let execution = Execution::from_config(config.unwrap());
}
