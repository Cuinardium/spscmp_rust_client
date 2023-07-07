use std::ffi::OsString;

use super::*;

#[test]
fn test_config_build() {
    let args = vec![
        OsString::from("-p"),
        OsString::from("8889"),
        OsString::from("-t"),
        OsString::from("hola"),
        OsString::from("adduser"),
        OsString::from("user"),
        OsString::from("pass"),
    ];

    let args = Arguments::from_vec(args);

    let config = Config::build(args, Option::from(String::from("hola"))).unwrap();

    assert_eq!(config.port, 8889);
    assert_eq!(config.auth_token, "hola");
    assert_eq!(
        config.command,
        Command::AddUser {
            name: OsString::from("user"),
            password: OsString::from("pass")
        }
    );
}

#[test]
fn test_config_build_no_port() {
    let args = vec![
        OsString::from("-t"),
        OsString::from("chau"),
        OsString::from("adduser"),
        OsString::from("user"),
        OsString::from("pass"),
    ];
    let args = Arguments::from_vec(args);

    let config = Config::build(args, Option::from(String::from("hola"))).unwrap();

    assert_eq!(config.port, DEFAULT_PORT);
    assert_eq!(config.auth_token, "chau");
    assert_eq!(
        config.command,
        Command::AddUser {
            name: OsString::from("user"),
            password: OsString::from("pass")
        }
    );
}

#[test]
fn test_config_build_no_auth_token() {
    let args = vec![
        OsString::from("-p"),
        OsString::from("8889"),
        OsString::from("adduser"),
        OsString::from("user"),
        OsString::from("pass"),
    ];
    let args = Arguments::from_vec(args);

    let config = Config::build(args, Option::from(String::from("hola"))).unwrap();

    assert_eq!(config.port, 8889);
    assert_eq!(config.auth_token, "hola");
    assert_eq!(
        config.command,
        Command::AddUser {
            name: OsString::from("user"),
            password: OsString::from("pass")
        }
    );
}

#[test]
fn test_config_build_no_port_no_auth_token() {
    let args = vec![
        OsString::from("adduser"),
        OsString::from("user"),
        OsString::from("pass"),
    ];
    let args = Arguments::from_vec(args);

    let config = Config::build(args, Option::from(String::from("hola"))).unwrap();

    assert_eq!(config.port, DEFAULT_PORT);
    assert_eq!(config.auth_token, "hola");
    assert_eq!(
        config.command,
        Command::AddUser {
            name: OsString::from("user"),
            password: OsString::from("pass")
        }
    );
}

#[test]
fn test_config_build_auth_token_no_default() {
    let args = vec![
        OsString::from("-p"),
        OsString::from("8889"),
        OsString::from("-t"),
        OsString::from("chau"),
        OsString::from("adduser"),
        OsString::from("user"),
        OsString::from("pass"),
    ];
    let args = Arguments::from_vec(args);

    let config = Config::build(args, None).unwrap();

    assert_eq!(config.port, 8889);
    assert_eq!(config.auth_token, "chau");
    assert_eq!(
        config.command,
        Command::AddUser {
            name: OsString::from("user"),
            password: OsString::from("pass")
        }
    );
}
