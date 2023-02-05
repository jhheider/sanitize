use anyhow::{Context, Result};
use std::{
    fs::{remove_dir, remove_file},
    io::BufRead,
};

use super::Execution;
impl Execution {
    pub fn run(&self, mut stdin: Box<dyn BufRead>) -> Result<()> {
        self.config.logger.verbose(format!(
            "preparing to sanitize {}",
            self.config.path.display()
        ));
        if self.files.is_empty() {
            self.config
                .logger
                .info(format!("{} is already clean", self.config.path.display()));
            return Ok(());
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
            return Ok(());
        }
        if !self.config.yes {
            self.config.logger.info("continue? [y/N] ");
            let mut yn = String::new();
            stdin
                .read_line(&mut yn)
                .context("can't read from terminal")?;
            if !yn.starts_with('y') {
                self.config.logger.info("aborting");
                return Ok(());
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
