mod log;
pub mod loglevel;

use std::fmt::Debug;

use crate::logger::loglevel::LogLevel;

#[derive(Clone)]
pub struct Logger {
    pub verbosity: LogLevel,
}

impl Logger {
    pub fn new(verbosity: LogLevel) -> Self {
        Self { verbosity }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(LogLevel::default())
    }
}

impl Debug for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Logger")
            .field("verbosity", &self.verbosity)
            .finish()
    }
}
