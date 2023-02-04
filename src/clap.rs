use std::path::PathBuf;

use clap::{command, value_parser, Arg, ArgAction, Command};

pub fn setup() -> Command {
    command!()
        .arg_required_else_help(true)
        .arg(
            Arg::new("path")
                .help("the directory to sanitize")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("file")
                .help(
                    "the entries to keep, one per line\n\
                         if not passed, will use stdin",
                )
                .short('f')
                .long("file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("dry_run")
                .short('n')
                .long("dry-run")
                .help("don't actually delete anything")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("regex")
                .short('r')
                .long("regex")
                .help("use regex to match entries")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("yes")
                .short('y')
                .long("yes")
                .help("don't ask for confirmation")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::Count),
        )
}
