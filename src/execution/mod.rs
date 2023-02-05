use std::path::PathBuf;

use crate::config::Config;

mod from_config;
mod run;

pub struct Execution {
    files: Vec<PathBuf>,
    config: Config,
}
