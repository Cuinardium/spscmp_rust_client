use pico_args::Arguments;
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

    let config = Config::build(args, String::from("hola")).unwrap_or_else(|_| {
        eprintln!("{HELP_MSG}");
        std::process::exit(1);
    });

    println!("Command: {:?}", config.command);
    println!("Port: {}", config.port);
    println!("Auth Token: {}", config.auth_token);
}
