use std::{error::Error, io::{Write, Read}, net::TcpStream};

use command::CommandResponse;

use crate::command::Command;

pub mod config;

mod command;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {

    // Open a TCP connection to the server
    let mut stream = TcpStream::connect(format!("localhost:{}", config.port))
        .map_err(|_| "Failed to connect to server")?;

    // Send the command
    stream
        .write_all(
            config
                .command
                .to_spscmp_command(&config.auth_token)
                .as_bytes(),
        )
        .map_err(|_| "Failed to send command to server")?;

    // Read the response
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|_| "Failed to read response from server")?;

    // Parse the response
    let response = Command::parse_spscmp_response(&config.command, &response)
        .map_err(|_| "Received invalid response from server")?;

    print_response(&response, &config.command);

    Ok(())
}


fn print_response(response: &CommandResponse, command: &Command) {
    match response {
        CommandResponse::Ok => println!("Command '{}' executed successfully", command),
        CommandResponse::Error => println!("Command '{}' failed", command),
        CommandResponse::UserList { users } => {
            println!("Users:");
            for user in users {
                println!("  {}", user);
            }
        }
        CommandResponse::Metrics { current_connections, total_connections, total_bytes_transferred } => {
            println!("Current connections: {}", current_connections);
            println!("Total connections: {}", total_connections);
            println!("Total bytes transferred: {}", total_bytes_transferred);
        },
        CommandResponse::Logs(logs) => {
            println!("Logs:");
            for log in logs {
                println!("  User {} logged in on {} at {}", log.name, log.date, log.time);
            }
        }
    }
}
