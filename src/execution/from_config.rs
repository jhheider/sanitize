use crate::config::Config;
use anyhow::Result;
use ignore::{overrides::OverrideBuilder, WalkBuilder};

use super::Execution;

impl Execution {
    pub fn from_config(config: &Config) -> Result<Self> {
        let mut overrides = OverrideBuilder::new(&config.path);
        for exclusion in &config.exclusions {
            config.logger.trace(format!("excluding {exclusion}"));
            overrides.add(&format!("!{exclusion}"))?;
        }
        let overrides = overrides.build()?;

        let mut files = Vec::new();

        for (idx, entry) in WalkBuilder::new(&config.path)
            .standard_filters(false)
            .overrides(overrides)
            .build()
            .enumerate()
        {
            if idx % 100_000 == 0 {
                config.logger.info(format!("scanned {idx} files"));
            }
            if entry.is_err() {
                config
                    .logger
                    .warn(format!("failed to read file: {}", entry.unwrap_err()));
                continue;
            }
            let entry = entry.unwrap();

            if entry.path() == config.path {
                continue;
            }
            files.push(entry.path().to_owned());
        }
        files.reverse();

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
}
