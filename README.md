![GitHub release (latest by date)](https://img.shields.io/github/v/release/jhheider/sanitize)
[![Test with Code Coverage](https://github.com/jhheider/sanitize/actions/workflows/test.yml/badge.svg)](https://github.com/jhheider/sanitize/actions/workflows/test.yml)
[![Check and Lint](https://github.com/jhheider/sanitize/actions/workflows/check-and-lint.yaml/badge.svg)](https://github.com/jhheider/sanitize/actions/workflows/check-and-lint.yaml)
[![Coverage Status](https://coveralls.io/repos/github/jhheider/sanitize/badge.svg?branch=main)](https://coveralls.io/github/jhheider/sanitize?branch=main)

# sanitize

Simple command-line tool to sanitize a directory to a specific whitelist,
written in Rust.

## Install

`cargo install sanitize` or, for [tea](https://tea.xyz) users,
`tea +crates.io/sanitize true`.

## Usage

Well, thanks to [clap](https://github.com/clap-rs/clap), the help system flows
nicely from the implementation:

```sh
A simple command line tool for sanitizing a directory

Usage: sanitize [OPTIONS] [path]

Arguments:
  [path]  the directory to sanitize

Options:
  -f, --file <file>  the entries to keep, one per line
                     if not passed, will use stdin
                     format is the same as .*ignore files
  -n, --dry-run      don't actually delete anything
  -y, --yes          don't ask for confirmation (dangerous, but very, very useful)
      --unsafe       allow unsafe operation (sanitize $HOME)
  -v, --verbose...   increase verbosity (can be used multiple times)
  -h, --help         Print help
  -V, --version      Print version
```
