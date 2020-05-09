use super::*;

#[test]
fn test_from_toml() {
    let s = include_str!("./role1.toml");

    let _: Role = toml::from_str(s).unwrap();
}
