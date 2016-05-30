use std::{io, convert};
use std::io::Read;
use std::path::Path;
use std::fs::File;

use rustc_serialize::Decodable;

use toml;


// TODO: Add option type to this to allow optional values.
#[derive(Debug, RustcDecodable)]
pub struct Config {
    pub environment: EnvironmentConfig,
    pub github: GithubConfig,
    pub server: ServerConfig,
}

#[derive(Debug, RustcDecodable)]
pub struct EnvironmentConfig {
    pub environment_name: String,
    pub log_config: String,
}

#[derive(Debug, RustcDecodable)]
pub struct GithubConfig{
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, RustcDecodable)]
pub struct ServerConfig {
    pub root_url: String,
    pub host_ip: String,
    pub host_port: u16,
}

#[derive(Debug)]
pub enum ConfigError {
    LoadIoError(io::Error),
    ParseError(toml::ParserError),
    DecodeError(toml::DecodeError),
}

impl convert::From<io::Error> for ConfigError {
	fn from(err: io::Error) -> ConfigError {
		ConfigError::LoadIoError(err)
	}
}

impl From<toml::DecodeError> for ConfigError {
	fn from(err: toml::DecodeError) -> ConfigError {
		ConfigError::DecodeError(err)
	}
}


impl Config {

    pub fn new(config_file_path: &Path) -> Result<Config, ConfigError> {
        let toml_value = load_file(config_file_path)?;
        let mut decoder = toml::Decoder::new(toml_value);
        let config = Config::decode(&mut decoder)?;
        Ok(config)
    }
}

fn load_file(toml_file_path: &Path) -> Result<toml::Value, ConfigError> {
    let mut toml_file = File::open(toml_file_path)?;
    let mut config_contents = String::new();
    toml_file.read_to_string(&mut config_contents)?;

    let mut parser = toml::Parser::new(&config_contents);
    match parser.parse() {
        Some(toml) => Ok(toml::Value::Table(toml)),
        None => Err(ConfigError::ParseError(parser.errors.pop().unwrap())),
    }
}
