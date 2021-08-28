use super::*;

#[test]
fn test_from_toml() {
    let s = include_str!("./role1.toml");

    let _: Role = toml::from_str(s).unwrap();
}

#[test]
fn test_from_yaml() {
    let s = include_str!("./role1.yaml");

    let _: Role = serde_yaml::from_str(s).unwrap();
}