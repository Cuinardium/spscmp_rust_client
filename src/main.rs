use pico_args::{Arguments, Error};
use spscmp_client::config::Config;

const HELP_MSG: &str = "Usage: spscmp_rust [options] <command>
Options:
    -p <port>       Port to listen on
    -t <token>      Authentication token
Commands:
    adduser <name> <password>    Add a new user
    deluser <name>               Delete a user
    updatepass <name> <password> Update a user's password
    updatename <name> <newname>  Update a user's name
    listusers                    List all users
    metrics                      Get server metrics
    logs                         Get server logs
    maxusers <num>               Set the maximum number of users
    maxconns <num>               Set the maximum number of connections";

fn main() {
    let args = Arguments::from_env();
    let option_auth_token_env = std::env::var("SPSCMP_AUTH_TOKEN").ok();

    let config = Config::build(args, option_auth_token_env).unwrap_or_else(|e| {

        match e {
            Error::NonUtf8Argument => eprintln!("Non-UTF8 argument found"),
            Error::MissingArgument => eprintln!("Missing command"),
            Error::MissingOption(_) => eprintln!("No authentication token provided, either use -t or set the SPSCMP_AUTH_TOKEN environment variable"),
            Error::OptionWithoutAValue(msg) => eprintln!("{msg}"),
            Error::Utf8ArgumentParsingFailed { value, cause } => eprintln!("Failed to parse command argument {value}: {cause}"),
            Error::ArgumentParsingFailed { cause } => eprintln!("Failed to parse command: {cause}"),
        }

        eprintln!("{HELP_MSG}");
        std::process::exit(1);
    });

    if let Err(e) = spscmp_client::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
