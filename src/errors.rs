use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;
use std::error::Error;
use std::convert::From;

use toml;

#[derive(Debug)]
pub enum ConfigError {
    LoadIoError(IoError),
    ParseError(toml::ParserError),
    DecodeError(toml::DecodeError),
}

// Probably unnecessary, just trying out serious error handling
impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::LoadIoError(_) => "An error occurred when loading the configuration file",
            ConfigError::ParseError(_) => "An error occurred when parsing the configuration file",
            ConfigError::DecodeError(_) => "An error occurred when decoding the configuration file",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConfigError::LoadIoError(ref error) => Some(error as &Error),
            _ => None,
        }
    }
}



impl From<IoError> for ConfigError {
	fn from(err: IoError) -> ConfigError {
		ConfigError::LoadIoError(err)
	}
}

impl From<toml::DecodeError> for ConfigError {
	fn from(err: toml::DecodeError) -> ConfigError {
		ConfigError::DecodeError(err)
	}
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

#[derive(Debug)]
pub struct APIError {
    error: String,
}
