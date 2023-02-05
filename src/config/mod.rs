use std::path::PathBuf;

use crate::logger::Logger;

mod from_matches;

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub exclusions: Vec<String>,
    pub dry_run: bool,
    pub yes: bool,
    pub logger: Logger,
}
