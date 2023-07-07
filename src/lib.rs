use std::error::Error;

pub mod config;

mod command;


pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {

    println!("Port: {}", config.port);
    println!("Token: {}", config.auth_token);
    println!("Command: {}", config.command.to_spscmp_command(&config.auth_token));
    
    Ok(())
}
