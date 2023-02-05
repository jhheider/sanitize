mod args;
mod config;
mod execution;
mod logger;
#[cfg(test)]
mod tests;

use crate::config::Config;
use crate::execution::Execution;
use colored::Colorize;
use std::process::exit;

#[cfg(not(tarpaulin_include))]
fn main() {
    use std::io::stdin;

    let setup = args::setup();
    let matches = setup.get_matches();
    let config = Config::from_matches(&matches).unwrap_or_else(|err| {
        eprintln!("[{}]: {:?}", "error".red().bold(), err);
        exit(1);
    });

    let execution = Execution::from_config(&config).unwrap_or_else(|err| {
        config.logger.error(err);
        exit(1);
    });

    execution
        .run(Box::new(stdin().lock()))
        .unwrap_or_else(|err| {
            config.logger.error(err);
            exit(1);
        });
}
