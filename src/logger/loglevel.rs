use std::fmt::{Display, Formatter, Result};

use colored::Colorize;

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Verbose,
    Debug,
    Trace,
}

impl From<u8> for LogLevel {
    fn from(level: u8) -> Self {
        match level {
            0 => LogLevel::Info,
            1 => LogLevel::Verbose,
            2 => LogLevel::Debug,
            3 => LogLevel::Trace,
            _ => LogLevel::Info,
        }
    }
}

impl From<LogLevel> for i8 {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => -2,
            LogLevel::Warning => -1,
            LogLevel::Info => 0,
            LogLevel::Verbose => 1,
            LogLevel::Debug => 2,
            LogLevel::Trace => 3,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                LogLevel::Error => "error".red().bold(),
                LogLevel::Warning => "warning".yellow().bold(),
                LogLevel::Info => "info".blue().bold(),
                LogLevel::Verbose => "verbose".green().bold(),
                LogLevel::Debug => "debug".magenta().bold(),
                LogLevel::Trace => "trace".white().bold(),
            },
        ))
    }
}
