//! Configuration related structures
use failure::Fallible;
use serde::Deserialize;
use std::{fs::read_to_string, path::PathBuf};
use toml;

#[derive(Clone, Deserialize)]
/// The global configuration
pub struct Config {
    /// The server configuration
    pub backend: BackendConfig,
    /// The frontend server configuration
    pub frontend: FrontendConfig,
    /// The logger configuration
    pub log: LogConfig,
    /// The database configuration
    pub mongodb: MongoConfig,
}

impl Config {
    /// Creates a new `Config` instance using the parameters found in the given
    /// TOML configuration file. If the file could not be found or the file is
    /// invalid, an `Error` will be returned.
    pub fn from_file(filename: &str) -> Fallible<Self> {
        Ok(toml::from_str(&read_to_string(filename)?)?)
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// The server configuration
pub struct BackendConfig {
    /// The full server URL
    pub url: String,
    /// The server certificate
    pub cert: PathBuf,
    /// The server key
    pub key: PathBuf,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// The server configuration
pub struct FrontendConfig {
    /// The server IP
    pub ip: String,
    /// The server port
    pub port: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// The logger configuration
pub struct LogConfig {
    /// The logging level of actix-web
    pub actix_web: String,
    /// The logging level of the application
    pub webapp: String,
}

#[derive(Clone, Deserialize)]
/// The database configuration
pub struct MongoConfig {
    /// The full host to the database
    pub host: String,
    /// The username for the database
    pub username: String,
    /// The password for the database
    pub password: String,
    /// The database to be used
    pub database: String,
}

