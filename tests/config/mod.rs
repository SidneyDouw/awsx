use crate::tools::fixture_path;
use awsx::config_old::{Config, Options};

mod options;

#[test]
fn get_exact_config_values() {
    let path = fixture_path("config/nested/config.toml");
    let config = Config::from_path(path, Default::default()).unwrap();

    assert_eq!(config.get("non_existent"), None);

    let v = config.get("sub").unwrap();
    assert!(matches!(v, toml::Value::Table(_)));
    assert!(v.as_table().unwrap().contains_key("a"));
    assert!(v.as_table().unwrap().contains_key("b"));
    assert!(v.as_table().unwrap().contains_key("var_a"));

    let v = config.get("sub.a").unwrap();
    assert!(matches!(v, toml::Value::Table(_)));
    assert!(v.as_table().unwrap().contains_key("var_b"));

    let v = config.get("sub.a.var_b").unwrap();
    assert!(matches!(v, toml::Value::String(_)));
    assert_eq!(v.as_str().unwrap(), "uvw");

    assert_eq!(config.get("var_a.b"), None);
}

#[test]
fn mody_config_values() {
    let path = fixture_path("config/nested/config.toml");
    let mut config = Config::from_path(path, Default::default()).unwrap();

    assert_eq!(config.get_mut("non_existent"), None);

    match config.get_string_mut("var_a") {
        Some(s) => s.push_str("def"),
        _ => unreachable!(),
    }
    assert_eq!(config.get_string("var_a"), Some(&"abcdef".to_string()));

    match config.get_string_mut("sub.var_a") {
        Some(s) => s.push_str("zyx"),
        _ => unreachable!(),
    }
    assert_eq!(config.get_string("sub.var_a"), Some(&"xyzzyx".to_string()));

    match config.get_string_mut("sub.a.var_b") {
        Some(s) => *s = "xxx".to_string(),
        _ => unreachable!(),
    }
    assert_eq!(config.get_string("sub.a.var_b"), Some(&"xxx".to_string()));
}

#[test]
fn load_outer_config() {
    let path = fixture_path("config/nested/config.toml");
    let config = Config::from_path(path, Default::default()).unwrap();

    assert_eq!(config.get("non_existent"), None);

    assert_eq!(unpack_str(config.get("envs.AWS_PROFILE")), Some("default"));
    assert_eq!(
        unpack_str(config.get("envs.AWS_DEFAULT_REGION")),
        Some("eu-central-1")
    );

    assert_eq!(unpack_str(config.get("var_a")), Some("abc"));
    assert_eq!(unpack_str(config.get("var_b")), Some("def"));
    assert_eq!(unpack_str(config.get("var_c")), Some("ghi"));

    assert_eq!(unpack_str(config.get("sub.var_a")), Some("xyz"));
    assert_eq!(unpack_str(config.get("sub.a.var_b")), Some("uvw"));
    assert_eq!(unpack_str(config.get("sub.b.var_c")), Some("rst"));

    assert!(matches!(config.get("sub"), Some(toml::Value::Table(_))));
    assert!(matches!(config.get("sub."), Some(toml::Value::Table(_))));
    assert!(matches!(config.get("sub.a"), Some(toml::Value::Table(_))));
    assert!(matches!(config.get("sub.b"), Some(toml::Value::Table(_))));
}

#[test]
fn load_inner_config() {
    let path = fixture_path("config/nested/sub/config.toml");
    let options = Options {
        nested: false,
        ..Default::default()
    };
    let config = Config::from_path(path, options).unwrap();

    assert_eq!(config.get("non_existent"), None);

    assert_eq!(unpack_str(config.get("envs.AWS_PROFILE")), Some("edited"));
    assert_eq!(unpack_str(config.get("var_a")), Some("cba"));
    assert_eq!(unpack_str(config.get("sub.b.var_c")), Some("sph"));

    assert!(matches!(config.get("sub"), Some(toml::Value::Table(_))));
    assert_eq!(config.get("sub.a"), None);
    assert!(matches!(config.get("sub.b"), Some(toml::Value::Table(_))));
}

#[test]
fn load_nested_configs() {
    let path = fixture_path("config/nested/sub/config.toml");
    let config = Config::from_path(path, Default::default()).unwrap();

    assert_eq!(config.get("non_existent"), None);

    assert_eq!(unpack_str(config.get("envs.AWS_PROFILE")), Some("edited"));
    assert_eq!(
        unpack_str(config.get("envs.AWS_DEFAULT_REGION")),
        Some("eu-central-1")
    );

    assert_eq!(unpack_str(config.get("var_a")), Some("cba"));
    assert_eq!(unpack_str(config.get("var_b")), Some("def"));
    assert_eq!(unpack_str(config.get("var_c")), Some("ghi"));

    assert_eq!(unpack_str(config.get("sub.var_a")), Some("xyz"));
    assert_eq!(unpack_str(config.get("sub.a.var_b")), Some("uvw"));
    assert_eq!(unpack_str(config.get("sub.b.var_c")), Some("sph"));

    assert!(matches!(config.get("sub"), Some(toml::Value::Table(_))));
    assert!(matches!(config.get("sub."), Some(toml::Value::Table(_))));
    assert!(matches!(config.get("sub.a"), Some(toml::Value::Table(_))));
    assert!(matches!(config.get("sub.b"), Some(toml::Value::Table(_))));
}

#[test]
fn load_nested_configs_with_overrides() {
    let path = fixture_path("config/nested/sub/config.toml");
    let mut config = Config::from_path(path, Default::default()).unwrap();

    config.set_string("var_b", "bbb");
    config.set_string("dynamic", "testing");
    config.set_string("dynamic", "test");
    config.set_string("sub.a.var_b", "sub_a_bbb");
    config.set_string("sub.c.var_a", "sub_c_aaa");
    config.set_string("sub.a.var_c", "sub_a_ccc");

    // println!("{:#?}", config);

    assert_eq!(config.get("non_existent"), None);

    assert_eq!(unpack_str(config.get("dynamic")), Some("test"));

    assert_eq!(unpack_str(config.get("envs.AWS_PROFILE")), Some("edited"));
    assert_eq!(
        unpack_str(config.get("envs.AWS_DEFAULT_REGION")),
        Some("eu-central-1")
    );

    assert_eq!(unpack_str(config.get("var_a")), Some("cba"));
    assert_eq!(unpack_str(config.get("var_b")), Some("bbb"));
    assert_eq!(unpack_str(config.get("var_c")), Some("ghi"));

    assert_eq!(unpack_str(config.get("sub.var_a")), Some("xyz"));
    assert_eq!(unpack_str(config.get("sub.a.var_b")), Some("sub_a_bbb"));
    assert_eq!(unpack_str(config.get("sub.a.var_c")), Some("sub_a_ccc"));
    assert_eq!(unpack_str(config.get("sub.b.var_c")), Some("sph"));
    assert_eq!(unpack_str(config.get("sub.c.var_a")), Some("sub_c_aaa"));
}

fn unpack_str(key: Option<&toml::Value>) -> Option<&str> {
    if let Some(v) = key {
        match v {
            toml::Value::String(v) => Some(v),
            _ => unreachable!("value of key {:?} is not a string", key),
        }
    } else {
        None
    }
}
