mod util {
    mod extract_parameter_keys_from_template {
        use crate::tools::fixture_path;
        use awsx::stack::util::extract_parameter_keys_from_template;

        #[test]
        fn with_valid_path() {
            let template = fixture_path("template.yml");
            let parameters = extract_parameter_keys_from_template(template).unwrap();

            assert_eq!(parameters, vec!["Test1", "Test2", "Test3"]);
        }

        #[test]
        fn with_invalid_path() {
            let template = fixture_path("wrong_file_name.yml");
            let r = extract_parameter_keys_from_template(template);

            assert!(matches!(
                r,
                Err(awsx::stack::Error::Io { path: _, source: _ })
            ));
        }
    }

    mod get_parameter_values_from_config {
        use crate::tools::fixture_path;
        use awsx::{config::Config, stack::util::get_parameter_values_from_config};
        use std::collections::HashMap;

        #[test]
        fn test() {
            let fixture = fixture_path("config_1");
            let template = fixture.join("template.yml");
            let config_path = fixture.join("config.toml");
            let config = Config::from_path(&config_path).unwrap();

            let actual = get_parameter_values_from_config(template, &config).unwrap();

            assert_eq!(
                actual,
                HashMap::from([
                    ("Test1".to_string(), "test_1".to_string()),
                    ("Test2".to_string(), "test_2".to_string()),
                    ("Test3".to_string(), "test_3".to_string()),
                    ("Test4".to_string(), "test_4".to_string()),
                    ("Test5".to_string(), "test_5".to_string()),
                ])
            );
        }
    }
}

// mod cli {
//     mod validate {
//         use crate::tools::fixture_path;
//         use awsx::{config2::Config2, stack::validate};
//
//         #[test]
//         #[should_panic]
//         fn invalid_template() {
//             let template = fixture_path("invalid_cf.yml");
//             let mut config = Config2::new();
//             config.set_string("env.AWS_PROFILE", "default");
//             config.set_string("env.AWS_DEFAULT_REGION", "eu-central-1");
//             config.set_bool("cmd.silent", true);
//
//             validate(template, &config).unwrap();
//         }
//
//         #[test]
//         fn valid_template() {
//             let template = fixture_path("template.yml");
//             let mut config = Config2::new();
//             config.set_string("env.AWS_PROFILE", "default");
//             config.set_string("env.AWS_DEFAULT_REGION", "eu-central-1");
//             config.set_bool("cmd.silent", true);
//
//             validate(template, &config).unwrap();
//         }
//     }
// }
