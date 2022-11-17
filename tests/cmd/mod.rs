use awsx::{
    cmd::{read, read_with_dir, run},
    config::Config,
};
use std::path::PathBuf;

#[test]
fn can_run_cmd() {
    let cmd = "echo testing";
    let mut config = Config::new();
    config.set_string("env.AWS_PROFILE", "default");
    config.set_string("env.AWS_DEFAULT_REGION", "eu-central-1");
    config.set_bool("cmd.silent", true);

    run(cmd, &config).unwrap();
}

#[test]
fn can_run_cmd_with_output() {
    let cmd = "echo testing";
    let mut config = Config::new();
    config.set_string("env.AWS_PROFILE", "default");
    config.set_string("env.AWS_DEFAULT_REGION", "eu-central-1");

    let actual = read(cmd, &config).unwrap();

    assert_eq!(actual, "testing");
}

#[test]
fn can_run_cmd_with_workdir() {
    let cmd = "echo $PWD";
    let workdir = std::fs::canonicalize("tests").unwrap();
    let mut config = Config::new();
    config.set_string("env.AWS_PROFILE", "default");
    config.set_string("env.AWS_DEFAULT_REGION", "eu-central-1");

    let actual = read_with_dir(cmd, &config, workdir).unwrap();

    let expected = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR should be set"),
    )
    .join("tests");
    assert_eq!(actual, expected.to_string_lossy());
}
