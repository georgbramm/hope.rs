
#[macro_use]
extern crate clap;
extern crate openssl;
use clap::{crate_version, App};
use failure::{format_err, Fallible};
use std::env::set_var;
use std::error::Error;
use std::fmt;
use std::process;
use hope::config::Config;
use hope_library::websocket::HopeWebSocket;
pub mod server;
use server::Server;

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
    // Initialize the logger
    env_logger::init();
    // Create and start the server
    let server = Server::from_config(&config)?;

    // Start the server
    server.start()?;

    Ok(())
}
