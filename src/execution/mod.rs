use std::{
    fs::{remove_dir, remove_file},
    io::stdin,
    path::PathBuf,
    process::exit,
};

use crate::config::Config;
use anyhow::{Context, Result};
use ignore::{overrides::OverrideBuilder, WalkBuilder};

pub struct Execution {
    files: Vec<PathBuf>,
    config: Config,
}

impl Execution {
    pub fn from_config(config: &Config) -> Result<Self> {
        let mut overrides = OverrideBuilder::new(&config.path);
        for exclusion in &config.exclusions {
            config.logger.trace(format!("excluding {}", exclusion));
            overrides.add(&format!("!{}", exclusion))?;
        }
        let overrides = overrides.build()?;

        let files = WalkBuilder::new(&config.path)
            .standard_filters(false)
            .overrides(overrides)
            .build()
            .filter_map(|entry| Some(entry.ok()?.into_path()))
            .filter(|path| path != &config.path)
            .collect::<Vec<PathBuf>>()
            .into_iter()
            .rev()
            .collect::<Vec<PathBuf>>();
        config.logger.debug(format!(
            "found file(s) to sanitize:\n{}",
            files
                .iter()
                .map(|f| f.to_string_lossy())
                .collect::<Vec<_>>()
                .join("\n")
        ));
        Ok(Self {
            files,
            config: config.clone(),
        })
    }

    pub fn run(&self) -> Result<()> {
        self.config.logger.verbose(format!(
            "preparing to sanitize {}",
            self.config.path.display()
        ));
        if self.files.is_empty() {
            self.config
                .logger
                .info(format!("{} is already clean", self.config.path.display()));
            exit(0);
        }
        let notice = format!(
            "this will sanitize {} file(s):\n{}",
            self.files.len(),
            self.files
                .iter()
                .map(|f| f.to_string_lossy())
                .collect::<Vec<_>>()
                .join("\n")
        );
        self.config.logger.info(notice);
        if self.config.dry_run {
            self.config.logger.info("dry-run enabled, aborting");
            exit(0);
        }
        if !self.config.yes {
            self.config.logger.info("continue? [y/N] ");
            let mut yn = String::new();
            stdin()
                .read_line(&mut yn)
                .context("can't read from terminal")?;
            if !yn.starts_with('y') {
                self.config.logger.info("aborting");
                exit(0);
            }
        }
        for file in &self.files {
            self.config.logger.trace(format!("processing {:?}", file));
            if file.is_file() {
                self.config.logger.trace(format!("{:?} is a file", file));

                self.config
                    .logger
                    .verbose(format!("removing file {:?}", file));
                remove_file(file)?;
            } else if file.is_dir() {
                self.config
                    .logger
                    .trace(format!("{:?} is a directory", file));

                if file.read_dir()?.next().is_some() {
                    self.config
                        .logger
                        .debug(format!("directory {:?} is not empty, skipping", file));
                    continue;
                }
                self.config
                    .logger
                    .verbose(format!("removing directory {:?}", file));
                remove_dir(file)?;
            }
        }
        Ok(())
    }
}
