
//! The core library
//#![deny(missing_docs)]
#![allow(unknown_lints, proc_macro_derive_resolution_fallback)]
pub mod config;
/// information model of the ehOPE communication protocol
pub mod protocol;
/// The global config file name
pub const CONFIG_FILENAME: &str = "Config.toml";
