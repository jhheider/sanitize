use colored::Colorize;

use crate::logger::loglevel::LogLevel;
use crate::logger::Logger;

#[test]
fn test_logger() {
    let logger = Logger::default();
    assert_eq!(logger.verbosity, LogLevel::default());

    logger.error("test");
    logger.warn("test");
    logger.info("test");
    logger.verbose("test");
    logger.debug("test");
    logger.trace("test");

    assert_eq!(format!("{:?}", logger), "Logger { verbosity: Info }");
}

#[test]
fn test_loglevel() {
    let lla = LogLevel::from(0);
    let llb = LogLevel::from(1);
    let llc = LogLevel::from(2);
    let lld = LogLevel::from(3);
    assert_eq!(lla, LogLevel::Info);
    assert_eq!(llb, LogLevel::Verbose);
    assert_eq!(llc, LogLevel::Debug);
    assert_eq!(lld, LogLevel::Trace);

    assert_eq!(i8::from(LogLevel::Error), -2);
    assert_eq!(i8::from(LogLevel::Warning), -1);
    assert_eq!(i8::from(lla), 0);
    assert_eq!(i8::from(llb), 1);
    assert_eq!(i8::from(llc), 2);
    assert_eq!(i8::from(lld), 3);

    assert_eq!(
        LogLevel::Verbose.to_string(),
        "verbose".green().bold().to_string()
    );
    assert_eq!(
        LogLevel::Debug.to_string(),
        "debug".magenta().bold().to_string()
    );
    assert_eq!(
        LogLevel::Trace.to_string(),
        "trace".white().bold().to_string()
    );
}
