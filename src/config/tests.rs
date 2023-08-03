mod init {
    use crate::config::Config;
    use std::path::PathBuf;

    #[test]
    fn config() {
        const PRINT_OUTPUT: bool = false;

        let paths = [
            // test invalid unicode in path
            ("tests/fixtures/config/non_existent.toml", true),
            // how to trigger metadata error
            ("tests/fixtures/config/folder", true),
            // missing .git folder for root folder
            ("tests/fixtures/config/invalid.toml", true),
            ("tests/fixtures/config/wrong_parameters_type.toml", true),
            ("tests/fixtures/config/wrong_secrets_type.toml", true),
            ("tests/fixtures/config/self_referencing.toml", true),
            ("tests/fixtures/config/secret_non_existent.toml", true),
            ("tests/fixtures/config/secret_folder.toml", true),
            ("tests/fixtures/config/secret_invalid.toml", true),
            ("tests/fixtures/config/secret_with_secret.toml", true),
            ("tests/fixtures/config/secret_empty.toml", false),
            ("tests/fixtures/config/empty.toml", false),
        ]
        .into_iter()
        .map(|(p, b)| (PathBuf::from(p), b));

        for (path, should_error) in paths {
            match Config::from_path(&path) {
                Ok(v) => {
                    assert!(!should_error, "should error but got Config: {:#?}", v);
                    if PRINT_OUTPUT {
                        println!("-- OK\n{:?}\n{:?}\n--\n", path, v);
                    }
                }
                Err(e) => {
                    assert!(should_error, "should not error but got Error: {:#?}", e);
                    if PRINT_OUTPUT {
                        println!("-- ERROR\n{:?}\n{}\n--\n", path, e);
                    }
                }
            }
        }
    }
}

mod getters {
    use crate::config::Config;
    use std::path::PathBuf;

    #[test]
    fn get_envs() {
        let config = Config::from_path(&PathBuf::from(
            "tests/fixtures/config/nested/sub/config.toml",
        ))
        .unwrap();
        config.get_envs().unwrap();
    }
}
