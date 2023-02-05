mod log;
mod loglevel;

use crate::logger::loglevel::LogLevel;

#[derive(Debug)]
pub(crate) struct Logger {
    verbosity: LogLevel,
}

impl Logger {
    pub fn new(verbosity: LogLevel) -> Self {
        Self { verbosity }
    }
}
