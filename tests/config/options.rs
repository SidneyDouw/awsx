use awsx::config::Options;
use std::path::PathBuf;

#[test]
fn get_project_root_automatically() {
    let options = Options {
        project_root: None,
        ..Default::default()
    };

    let root_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set"),
    );

    let r = options.get_project_root();
    match r {
        Ok(path) => assert_eq!(
            path,
            root_dir.join("..").canonicalize().expect("valid path")
        ),
        Err(_) => unreachable!(),
    }
}

#[test]
fn get_project_root_with_invalid_path() {
    let options = Options {
        project_root: Some("tests/fixtures/nested".into()),
        ..Default::default()
    };

    let r = options.get_project_root();
    match r {
        Ok(_) => unreachable!(),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {}
            _ => unreachable!(),
        },
    }
}

#[test]
fn get_project_root_with_relative_path() {
    let options = Options {
        project_root: Some("tests/fixtures/nested_configs".into()),
        ..Default::default()
    };

    let root_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set"),
    );

    let r = options.get_project_root();
    match r {
        Ok(path) => assert_eq!(path, root_dir.join("tests/fixtures/nested_configs")),
        Err(_) => unreachable!(),
    }
}

#[test]
fn get_project_root_with_absolute_path() {
    let root_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set"),
    );

    let options = Options {
        project_root: Some(root_dir.join("../..").canonicalize().expect("valid path")),
        ..Default::default()
    };

    let r = options.get_project_root();
    match r {
        Ok(path) => {
            assert_eq!(
                path,
                root_dir.join("../..").canonicalize().expect("valid path")
            )
        }
        Err(_) => unreachable!(),
    }
}
