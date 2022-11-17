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

    mod parameters_to_string {
        use awsx::stack::util::parameters_to_string;
        use toml::Value;

        #[test]
        fn test() {
            let parameters = vec![
                ("test_str".to_string(), Value::String("abc".to_string())),
                ("test_int".to_string(), Value::Integer(123)),
                ("test_float".to_string(), Value::Float(123.0)),
                ("test_bool".to_string(), Value::Boolean(true)),
            ];
            let actual = parameters_to_string(parameters);

            let mut expected = "--parameters".to_string();
            expected.push_str(" ParameterKey=test_str,ParameterValue=\"abc\"");
            expected.push_str(" ParameterKey=test_int,ParameterValue=123");
            expected.push_str(" ParameterKey=test_float,ParameterValue=123.0");
            expected.push_str(" ParameterKey=test_bool,ParameterValue=true");

            assert_eq!(actual, expected)
        }
    }

    mod get_parameter_values_from_config {
        use crate::tools::fixture_path;
        use awsx::{config::Config, stack::util::get_parameter_values_from_config};
        use toml::Value;

        #[test]
        fn test() {
            let fixture = fixture_path("config_1");
            let template = fixture.join("template.yml");
            let config_path = fixture.join("config.toml");
            let config = Config::from_path(config_path, Default::default()).unwrap();

            let actual = get_parameter_values_from_config(template, &config).unwrap();

            assert_eq!(
                actual,
                vec![
                    ("Test1".to_string(), Value::String("test_1".to_string())),
                    ("Test2".to_string(), Value::String("test_2".to_string())),
                    ("Test3".to_string(), Value::String("test_3".to_string())),
                    ("Test4".to_string(), Value::String("test_4".to_string())),
                ]
            );
        }
    }
}

mod cli {
    mod validate {
        use crate::tools::fixture_path;
        use awsx::{config::Config, stack::validate};

        #[test]
        #[should_panic]
        fn invalid_template() {
            let template = fixture_path("invalid_cf.yml");
            let mut config = Config::new();
            config.set_string("env.AWS_PROFILE", "default");
            config.set_string("env.AWS_DEFAULT_REGION", "eu-central-1");
            config.set_bool("cmd.silent", true);

            validate(template, &config).unwrap();
        }

        #[test]
        fn valid_template() {
            let template = fixture_path("template.yml");
            let mut config = Config::new();
            config.set_string("env.AWS_PROFILE", "default");
            config.set_string("env.AWS_DEFAULT_REGION", "eu-central-1");
            config.set_bool("cmd.silent", true);

            validate(template, &config).unwrap();
        }
    }
}
