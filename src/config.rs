use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::ArgMatches;

use crate::logger::Logger;

#[derive(Debug, Clone)]
pub struct Config {
    pub path: PathBuf,
    pub exclusions: Vec<String>,
    pub dry_run: bool,
    pub yes: bool,
    pub logger: Logger,
}

impl Config {
    #[cfg(not(tarpaulin_include))]
    pub fn from_matches(matches: &ArgMatches) -> Result<Self> {
        use std::{
            fs::read_to_string,
            io::{stdin, BufRead},
        };

        use anyhow::Error;

        let verbosity = matches.get_count("verbose");

        let logger = Logger::new(verbosity.into());
        logger.verbose(format!("log level set to {verbosity}"));

        let path = matches
            .get_one::<PathBuf>("path")
            .context("path to sanitize is required")?
            .to_owned();

        logger.debug(format!("sanitizing directory {:?}", path));

        let dry_run = matches.get_flag("dry_run");
        if dry_run {
            logger.verbose("dry-run enabled");
        }

        let yes = matches.get_flag("yes");
        if yes {
            logger.verbose("confirmations bypassed");
            if dry_run {
                logger.warn("--yes has no effect with --dry-run");
            }
        }

        let file = matches.get_one::<PathBuf>("file");
        let exclusions = if let Some(file) = file {
            logger.verbose(format!("using ignore file {:?}", file));
            let file = read_to_string(file).context(format!("failed to read file {:?}", file))?;
            file.lines()
                .map(|line| {
                    logger.trace(format!("read exclusion: {}", line.trim()));
                    line.trim().to_owned()
                })
                .collect::<Vec<String>>()
        } else {
            if !yes {
                return Err(Error::msg(
                    "--yes is required when reading exclusions from stdin",
                ));
            }
            logger.verbose("reading exlusions from stdin".to_string());

            let mut exclusions = Vec::new();
            let stdin = stdin();
            let mut handle = stdin.lock();
            loop {
                let mut line = String::new();
                let bytes = handle
                    .read_line(&mut line)
                    .context("failed to read from stdin")?;
                if bytes == 0 {
                    break;
                }
                logger.trace(format!("read line: {}", line.trim()));
                exclusions.push(line.trim().to_owned());
            }
            exclusions
        };
        logger.verbose(format!("{} exclusions read", exclusions.len()));
        logger.debug(format!("exclusions: {}", exclusions.join("\n")));

        Ok(Config {
            path,
            exclusions,
            dry_run,
            yes,
            logger,
        })
    }
}
