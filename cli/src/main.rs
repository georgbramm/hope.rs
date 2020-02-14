#[macro_use]
extern crate clap;
use clap::{crate_version, App, ArgMatches};
use failure::{format_err, Fallible};
use std::env::set_var;
use std::error::Error;
use std::fmt;
use std::process;
use hope::config::Config;
use hope_library::scheme::hope as hopeScheme;
use hope_library::websocket::HopeWebSocket;
use hope_server::server::Server;

// Application commands
const CMD_SETUP: &'static str = "setup";
const CMD_KEYGEN: &'static str = "keygen";
const CMD_ENCRYPT: &'static str = "encrypt";

#[derive(Debug)]
struct ReclaimPathError {
    details: String,
}

impl ReclaimPathError {
    fn new(msg: &str) -> ReclaimPathError {
        ReclaimPathError { details: msg.to_string() }
    }
}

impl fmt::Display for ReclaimPathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.details)
    }
}

impl Error for ReclaimPathError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn main() -> Fallible<()> {
    // Load the CLI parameters from the yaml file
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    // Retrieve the config file path
    let config_filename = matches.value_of("config").ok_or_else(|| {
        format_err!("No 'config' provided")
    })?;

    // Parse the configuration
    let config = Config::from_file(config_filename)?;

    // Set the logging verbosity
    set_var(
        "RUST_LOG",
        format!(
            "actix_web={},webapp={},backend={}",
            config.log.actix_web,
            config.log.webapp,
            config.log.webapp
        ),
    );
    // Create the scheme
    let wsh = HopeWebSocket::new();
    let _hope: hopeScheme = hopeScheme::new(String::from("demo"), &wsh);


    if let Err(e) = run(matches, _hope) {
        println!("Application error: {}", e);
        process::exit(1);
    }

    fn run(matches: ArgMatches, _hope: hopeScheme) -> Result<(), ReclaimPathError> {
        match matches.subcommand() {
            (CMD_SETUP, Some(matches)) => run_setup(matches, _hope),
            (CMD_KEYGEN, Some(matches)) => run_keygen(matches, _hope),
            (CMD_ENCRYPT, Some(matches)) => run_encrypt(matches, _hope),
            _ => Ok(()),
        }
    }

    fn run_setup(arguments: &ArgMatches, _hope: hopeScheme) -> Result<(), ReclaimPathError> {
        println!("Running setup...");
        let sp = _hope.parameters();
        let jsp = serde_json::to_string(&sp);
        println!("{:?}", jsp);
        Ok(())
    }

    fn run_keygen(arguments: &ArgMatches, _hope: hopeScheme) -> Result<(), ReclaimPathError> {
        println!("Running keygen...");
        let sk = hopeScheme::<'_>::keygen().unwrap();
        let jsk = serde_json::to_string(&sk);
        println!("{:?}", jsk);
        Ok(())
    }

    fn run_encrypt(arguments: &ArgMatches, _hope: hopeScheme) -> Result<(), ReclaimPathError> {
        println!("Running encrypt...");
        Ok(())
    }
    Ok(())
}
