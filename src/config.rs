use pico_args::Arguments;

use crate::command::Command;

const DEFAULT_PORT: u16 = 8889;
const PORT_ARG_OPT: &str = "-p";
const AUTH_TOKEN_ARG_OPT: &str = "-t";

// ====================== CONFIG ======================

pub struct Config {
    pub command: Command,
    pub port: u16,
    pub auth_token: String,
}

impl Config {
    pub fn build(mut args: Arguments, auth_token_env: String) -> Result<Config, pico_args::Error> {
        let port = args
            .opt_value_from_str(PORT_ARG_OPT)?
            .unwrap_or(DEFAULT_PORT);

        let auth_token = args
            .opt_value_from_str(AUTH_TOKEN_ARG_OPT)?
            .unwrap_or(auth_token_env);

        let mut remaining = args.finish().into_iter();
        let command = Command::from_args(&mut remaining)?;

        Ok(Config {
            command,
            port,
            auth_token,
        })
    }
}

// ====================== TESTS ======================

#[cfg(test)]
mod test {
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

        let config = Config::build(args, String::from("hola")).unwrap();

        assert_eq!(config.port, 8889);
        assert_eq!(config.auth_token, "hola");
        assert_eq!(
            config.command,
            Command::AddUser(OsString::from("user"), OsString::from("pass"))
        );
    }

    #[test]
    fn test_config_build_no_port() {
        let args = vec![
            OsString::from("-t"),
            OsString::from("hola"),
            OsString::from("adduser"),
            OsString::from("user"),
            OsString::from("pass"),
        ];
        let args = Arguments::from_vec(args);

        let config = Config::build(args, String::from("hola")).unwrap();

        assert_eq!(config.port, DEFAULT_PORT);
        assert_eq!(config.auth_token, "hola");
        assert_eq!(
            config.command,
            Command::AddUser(OsString::from("user"), OsString::from("pass"))
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

        let config = Config::build(args, String::from("hola")).unwrap();

        assert_eq!(config.port, 8889);
        assert_eq!(config.auth_token, "hola");
        assert_eq!(
            config.command,
            Command::AddUser(OsString::from("user"), OsString::from("pass"))
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

        let config = Config::build(args, String::from("hola")).unwrap();

        assert_eq!(config.port, DEFAULT_PORT);
        assert_eq!(config.auth_token, "hola");
        assert_eq!(
            config.command,
            Command::AddUser(OsString::from("user"), OsString::from("pass"))
        );
    }
}
