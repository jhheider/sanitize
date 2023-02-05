use std::fmt::Display;

use crate::logger::LogLevel;

use super::Logger;

impl Logger {
    pub fn error(&self, message: impl Display) {
        self.log(message, LogLevel::Error)
    }

    pub fn warn(&self, message: impl Display) {
        self.log(message, LogLevel::Warning)
    }

    pub fn info(&self, message: impl Display) {
        self.log(message, LogLevel::Info)
    }

    pub fn verbose(&self, message: impl Display) {
        self.log(message, LogLevel::Verbose)
    }

    pub fn debug(&self, message: impl Display) {
        self.log(message, LogLevel::Debug)
    }

    pub fn trace(&self, message: impl Display) {
        self.log(message, LogLevel::Trace)
    }

    fn log(&self, message: impl Display, level: LogLevel) {
        match level {
            LogLevel::Error | LogLevel::Warning => eprintln!("[{level}]: {message}"),
            _ => {
                if self.verbosity >= level {
                    println!("[{level}]: {message}");
                }
            }
        }
    }
}
