use pico_args::{Arguments, Error, Keys};

use crate::command::Command;

const DEFAULT_HOST: &str = "localhost";
const HOST_ARG_OPT: &str = "-h";
const DEFAULT_PORT: u16 = 8889;
const PORT_ARG_OPT: &str = "-p";
const AUTH_TOKEN_ARG_OPT: &str = "-t";

// ====================== CONFIG ======================

pub struct Config {
    pub command: Command,
    pub host: String,
    pub port: u16,
    pub auth_token: String,
}

impl Config {
    pub fn build(
        mut args: Arguments,
        option_auth_token_env: Option<String>,
    ) -> Result<Config, Error> {
        let host = args
            .opt_value_from_str(HOST_ARG_OPT)?
            .unwrap_or(DEFAULT_HOST.to_string());

        let port = args
            .opt_value_from_str(PORT_ARG_OPT)?
            .unwrap_or(DEFAULT_PORT);

        let auth_token = args
            .opt_value_from_str(AUTH_TOKEN_ARG_OPT)?
            .or(option_auth_token_env)
            .ok_or(Error::MissingOption(Keys::from(AUTH_TOKEN_ARG_OPT)))?;

        let mut remaining = args.finish().into_iter();
        let command = Command::from_args(&mut remaining)?;

        Ok(Config {
            command,
            host,
            port,
            auth_token,
        })
    }
}

// ====================== TESTS ======================

#[cfg(test)]
mod test;
