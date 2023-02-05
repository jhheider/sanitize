use std::path::PathBuf;

use clap::{command, value_parser, Arg, ArgAction, Command};

#[cfg(not(tarpaulin_include))]
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
                         if not passed, will use stdin\n\
                         format is the same as .*ignore files",
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
            Arg::new("yes")
                .short('y')
                .long("yes")
                .help("don't ask for confirmation (dangerous, but very, very useful)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unsafe")
                .long("unsafe")
                .help("allow unsafe operation (sanitize $HOME)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("increase verbosity (can be used multiple times)")
                .action(ArgAction::Count),
        )
}
