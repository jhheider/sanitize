use std::path::PathBuf;

use crate::config::Config;

mod from_config;
mod run;

pub struct Execution {
    pub files: Vec<PathBuf>,
    config: Config,
}
