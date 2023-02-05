use std::{
    fs::{create_dir, File},
    path::PathBuf,
};

use tempdir::TempDir;

use crate::{config::Config, execution::Execution};

#[test]
fn test_execution_from_config() {
    let ca = Config {
        path: PathBuf::from("."),
        exclusions: vec![String::from("mod.rs")],
        ..Default::default()
    };
    let ea = Execution::from_config(&ca);

    assert!(ea.is_ok());
    assert!(!ea.unwrap().files.is_empty());

    let cb = Config {
        path: PathBuf::from("/root"),
        exclusions: vec![String::from("mod.rs")],
        ..Default::default()
    };
    let eb = Execution::from_config(&cb);

    assert!(&eb.is_ok());
    assert!(&eb.unwrap().files.is_empty());
}

#[test]
fn test_execution_run() {
    let stdin = Box::new(&b"y"[..]);
    let ca = Config {
        path: PathBuf::from("."),
        dry_run: true,
        exclusions: vec![String::from("mod.rs")],
        ..Default::default()
    };
    let ea = Execution::from_config(&ca).unwrap();

    assert!(ea.run(stdin.clone()).is_ok());

    if PathBuf::from("/root").metadata().is_ok() {
        // too risky
        panic!("can't run test_execution_run on a non-readonly filesystem");
    }

    let cb = Config {
        path: PathBuf::from("/root"),
        dry_run: true,
        exclusions: vec![String::from("mod.rs")],
        ..Default::default()
    };
    let eb = Execution::from_config(&cb).unwrap();

    assert!(&eb.run(stdin.clone()).is_ok());
    assert!(&eb.files.is_empty());

    let temp = TempDir::new("test").unwrap();
    File::create(temp.path().join("foo")).unwrap();
    create_dir(temp.path().join("bar")).unwrap();
    create_dir(temp.path().join("bat")).unwrap();
    File::create(temp.path().join("bar/foo")).unwrap();
    let cc = Config {
        path: temp.path().to_path_buf(),
        yes: false,
        exclusions: vec![String::from("bar/foo")],
        ..Default::default()
    };
    let ec = Execution::from_config(&cc).unwrap();

    assert!(ec.run(stdin.clone()).is_ok());

    let stdin2 = Box::new(&b"n"[..]);
    assert!(ec.run(stdin2.clone()).is_ok());
}
