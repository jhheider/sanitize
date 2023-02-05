use std::path::PathBuf;

use crate::config::Config;

pub struct Execution {
    files: Vec<PathBuf>,
}

impl Execution {
    pub fn from_config(config: Config) -> Self {
        let files = config
            .path
            .read_dir()
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        Self { files }
    }
}
