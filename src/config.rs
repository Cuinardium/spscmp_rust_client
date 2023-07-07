use std::ffi::OsString;

use pico_args::{Arguments, Error};

const DEFAULT_PORT: u16 = 8889;
const PORT_ARG_OPT: &str = "-p";
const AUTH_TOKEN_ARG_OPT: &str = "-t";


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

// ====================== COMMANDS ======================

//TODO: Implement commands
#[derive(Debug, PartialEq)]
pub enum Command {
    ADDUSER(OsString, OsString),
    DELUSER(OsString),
    UPDATEPASS(OsString, OsString),
    UPDATENAME(OsString, OsString),
    LISTUSERS,
    METRICS,
    LOGS,
    MAXUSERS(u32),
    MAXCONNS(u32),
}

impl Command {
    fn from_args(args: &mut impl Iterator<Item = OsString>) -> Result<Command, Error> {
        let command = args.next().ok_or(Error::MissingArgument)?;
        let command = command.to_str().ok_or(Error::ArgumentParsingFailed {
            cause: String::from("Invalid command"),
        })?;

        let command = match command {
            "adduser" => {
                let name = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No name provided for adduser"),
                })?;
                let password = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No password provided for adduser"),
                })?;

                Command::ADDUSER(name, password)
            }
            "deluser" => {
                let name = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No name provided for deluser"),
                })?;

                Command::DELUSER(name)
            }
            "updatepass" => {
                let name = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No name provided for updatepass"),
                })?;
                let password = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No password provided for updatepass"),
                })?;

                Command::UPDATEPASS(name, password)
            }
            "updatename" => {
                let name = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No name provided for updatename"),
                })?;
                let newname = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No new name provided for updatename"),
                })?;

                Command::UPDATENAME(name, newname)
            }
            "listusers" => Command::LISTUSERS,
            "metrics" => Command::METRICS,
            "logs" => Command::LOGS,
            "maxusers" => {
                let num = args
                    .next()
                    .ok_or(Error::ArgumentParsingFailed { cause: String::from("No number provided for maxusers") })?
                    .into_string()
                    .map_err(|_| Error::ArgumentParsingFailed { cause: String::from("Invalid number provided for maxusers") })?
                    .parse::<u32>()
                    .map_err(|_| Error::ArgumentParsingFailed { cause: String::from("Invalid number provided for maxusers") })?;

                Command::MAXUSERS(num)
            }
            "maxconns" => {
                let num = args
                    .next()
                    .ok_or(Error::ArgumentParsingFailed { cause: String::from("No number provided for maxconns") })?
                    .into_string()
                    .map_err(|_| Error::ArgumentParsingFailed { cause: String::from("Invalid number provided for maxconns") })?
                    .parse::<u32>()
                    .map_err(|_| Error::ArgumentParsingFailed { cause: String::from("Invalid number provided for maxconns") })?;
                Command::MAXCONNS(num)
            }
            _ => return Err(Error::ArgumentParsingFailed { cause: String::from("Invalid command") }),
        };

        Ok(command)
    }
}

// ====================== TESTS ======================

#[cfg(test)]
mod test {
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
            Command::ADDUSER(OsString::from("user"), OsString::from("pass"))
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
            Command::ADDUSER(OsString::from("user"), OsString::from("pass"))
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
            Command::ADDUSER(OsString::from("user"), OsString::from("pass"))
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
            Command::ADDUSER(OsString::from("user"), OsString::from("pass"))
        );
    }
}
