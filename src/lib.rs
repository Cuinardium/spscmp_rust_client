use std::{error::Error, io::{Write, Read}, net::TcpStream};

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

    println!("Response: {}", response);

    Ok(())
}
