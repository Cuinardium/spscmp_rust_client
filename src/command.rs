use std::ffi::OsString;

use pico_args::Error;

// ====================== COMMANDS ======================

#[derive(Debug, PartialEq)]
pub enum Command {
    AddUser {
        name: OsString,
        password: OsString,
    },
    DelUser {
        name: OsString,
    },
    UpdatePass {
        name: OsString,
        new_password: OsString,
    },
    UpdateName {
        name: OsString,
        new_name: OsString,
    },
    ListUsers,
    Metrics,
    Logs,
    MaxUsers {
        num: u32,
    },
    Maxconns {
        num: u32,
    },
}

impl Command {
    pub fn from_args(args: &mut impl Iterator<Item = OsString>) -> Result<Command, Error> {
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

                Command::AddUser { name, password }
            }
            "deluser" => {
                let name = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No name provided for deluser"),
                })?;

                Command::DelUser { name }
            }
            "updatepass" => {
                let name = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No name provided for updatepass"),
                })?;
                let new_password = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No password provided for updatepass"),
                })?;

                Command::UpdatePass { name, new_password }
            }
            "updatename" => {
                let name = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No name provided for updatename"),
                })?;
                let new_name = args.next().ok_or(Error::ArgumentParsingFailed {
                    cause: String::from("No new name provided for updatename"),
                })?;

                Command::UpdateName { name, new_name }
            }
            "listusers" => Command::ListUsers,
            "metrics" => Command::Metrics,
            "logs" => Command::Logs,
            "maxusers" => {
                let num = args
                    .next()
                    .ok_or(Error::ArgumentParsingFailed {
                        cause: String::from("No number provided for maxusers"),
                    })?
                    .into_string()
                    .map_err(|_| Error::ArgumentParsingFailed {
                        cause: String::from("Invalid number provided for maxusers"),
                    })?
                    .parse::<u32>()
                    .map_err(|_| Error::ArgumentParsingFailed {
                        cause: String::from("Invalid number provided for maxusers"),
                    })?;

                Command::MaxUsers { num }
            }
            "maxconns" => {
                let num = args
                    .next()
                    .ok_or(Error::ArgumentParsingFailed {
                        cause: String::from("No number provided for maxconns"),
                    })?
                    .into_string()
                    .map_err(|_| Error::ArgumentParsingFailed {
                        cause: String::from("Invalid number provided for maxconns"),
                    })?
                    .parse::<u32>()
                    .map_err(|_| Error::ArgumentParsingFailed {
                        cause: String::from("Invalid number provided for maxconns"),
                    })?;

                Command::Maxconns { num }
            }
            _ => {
                return Err(Error::ArgumentParsingFailed {
                    cause: String::from("Invalid command"),
                })
            }
        };

        Ok(command)
    }

    pub fn to_spscmp_command(&self, auth_token: &str) -> String {
        match self {
            Command::AddUser { name, password } => format!(
                "{} ADDUSER {} {}\r\n",
                auth_token,
                name.to_str().unwrap(),
                password.to_str().unwrap()
            ),
            Command::DelUser { name } => {
                format!("{} DELUSER {}\r\n", auth_token, name.to_str().unwrap())
            }
            Command::UpdatePass { name, new_password } => format!(
                "{} UPDATEPASS {} {}\r\n",
                auth_token,
                name.to_str().unwrap(),
                new_password.to_str().unwrap()
            ),
            Command::UpdateName { name, new_name } => format!(
                "{} UPDATENAME {} {}\r\n",
                auth_token,
                name.to_str().unwrap(),
                new_name.to_str().unwrap()
            ),
            Command::ListUsers => format!("{} LISTUSERS\r\n", auth_token),
            Command::Metrics => format!("{} METRICS\r\n", auth_token),
            Command::Logs => format!("{} LOGS\r\n", auth_token),
            Command::MaxUsers { num } => format!("{} MAXUSERS {}\r\n", auth_token, num),
            Command::Maxconns { num } => format!("{} MAXCONNS {}\r\n", auth_token, num),
        }
    }

    pub fn parse_spscmp_response(command: &Command, response: &str) -> Result<CommandResponse, ()> {
        let response = match command {
            Command::AddUser { .. }
            | Command::DelUser { .. }
            | Command::UpdatePass { .. }
            | Command::UpdateName { .. }
            | Command::MaxUsers { .. }
            | Command::Maxconns { .. } => match response {
                "OK\r\n" => CommandResponse::Ok,
                "ERR\r\n" => CommandResponse::Error,
                _ => return Err(()),
            },
            Command::ListUsers => {
                let mut lines = response.lines();

                let header = lines.next().ok_or(())?;
                match header {
                    "OK" => {}
                    "ERR" => return Ok(CommandResponse::Error),
                    _ => return Err(()),
                }

                let users = lines
                    .take_while(|&line| line != ".")
                    .map(String::from)
                    .collect();

                CommandResponse::UserList { users }
            }
            Command::Metrics => {
                let mut lines = response.lines();

                let header = lines.next().ok_or(())?;
                match header {
                    "OK" => {}
                    "ERR" => return Ok(CommandResponse::Error),
                    _ => return Err(()),
                }

                let current_connections = lines.next().ok_or(())?.parse::<u32>().map_err(|_| ())?;
                let total_connections = lines.next().ok_or(())?.parse::<u32>().map_err(|_| ())?;
                let total_bytes_transferred =
                    lines.next().ok_or(())?.parse::<u32>().map_err(|_| ())?;

                if lines.next().ok_or(())? != "." {
                    return Err(());
                }

                CommandResponse::Metrics {
                    current_connections,
                    total_connections,
                    total_bytes_transferred,
                }
            }
            Command::Logs => {
                let mut lines = response.lines();

                let header = lines.next().ok_or(())?;
                match header {
                    "OK" => {}
                    "ERR" => return Ok(CommandResponse::Error),
                    _ => return Err(()),
                }

                let logs = lines
                    .take_while(|&line| line != ".")
                    .map(|line| {
                        let mut parts = line.split_whitespace();

                        let name = parts.next().ok_or(())?.to_string();
                        let date = parts.next().ok_or(())?.to_string();
                        let time = parts.next().ok_or(())?.to_string();

                        // TODO: Check if date and time are valid

                        // No more parts should be present
                        if parts.next().is_some() {
                            return Err(());
                        }

                        Ok(UserLog { name, date, time })
                    })
                    .collect::<Result<Vec<UserLog>, ()>>()?;

                CommandResponse::Logs(logs)
            }
        };

        Ok(response)
    }
}

// =============== Command Response ===============
#[derive(Debug, PartialEq)]
pub enum CommandResponse {
    Ok,
    Error,
    UserList {
        users: Vec<String>,
    },
    Metrics {
        current_connections: u32,
        total_connections: u32,
        total_bytes_transferred: u32,
    },
    Logs(Vec<UserLog>),
}

// TODO: Convert date and time to a meaningful data
#[derive(Debug, PartialEq)]
pub struct UserLog {
    pub name: String,
    pub date: String,
    pub time: String,
}
