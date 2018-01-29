use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::io::Error as IoError;
use std::error::Error;
use std::convert::From;

use iron::modifier::Modifier;
use iron::response::Response;
use iron::status::Status;
use iron::error::IronError;

use toml;
use serde_json;

#[derive(Debug)]
pub enum ConfigError {
    LoadIoError(IoError),
    ParseError(toml::de::Error),
}

// Probably unnecessary, just trying out serious error handling
impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::LoadIoError(_) => "An error occurred when loading the configuration file",
            ConfigError::ParseError(_) => "An error occurred when parsing the configuration file",
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

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> ConfigError {
        ConfigError::ParseError(err)
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

// TODO: Serialize this to an actual error code?
// It seems to be serialized to object with name and empty array at the moment.
#[derive(Debug, Clone, Serialize)]
pub enum APIErrorCode {
    BadJSON,
    NoBody,
    Unauthorized,
    InvalidToken,
    GithubError,
    Unknown,
}

impl APIErrorCode {
    pub fn status(&self) -> Status {
        match *self {
            APIErrorCode::BadJSON => Status::BadRequest,
            APIErrorCode::NoBody => Status::UnprocessableEntity,
            APIErrorCode::Unauthorized => Status::Unauthorized,
            APIErrorCode::InvalidToken => Status::Unauthorized,
            APIErrorCode::GithubError => Status::InternalServerError,
            APIErrorCode::Unknown => Status::InternalServerError,
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct APIError {
    error: String,
    error_code: APIErrorCode,
}


impl APIError {
    pub fn new<E: Into<String>>(error: E, code: APIErrorCode) -> APIError {
        APIError {
            error: error.into(),
            error_code: code,
        }
    }
    pub fn no_body() -> APIError{
        APIError::new("No JSON body was specified on the request",
                     APIErrorCode::NoBody)
    }
    pub fn bad_json() -> APIError {
        APIError::new("Error while decoding JSON Body. Missing field?",
                     APIErrorCode::BadJSON)
    }

    pub fn github_error() -> APIError {
        APIError::new("An error occurred while communicating with Github",
                      APIErrorCode::GithubError)
    }

    pub fn status(&self) -> Status {
       self.error_code.status()
    }
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.error)
    }
}

impl Error for APIError {
    fn description(&self) -> &str {
        &self.error
    }
}

impl From<APIError> for IronError {
	fn from(err: APIError) -> IronError {
		IronError::new(err.clone(), err)
	}
}

// Sets the status and body of the response for an error.
impl Modifier<Response> for APIError {
    fn modify(self, response: &mut Response) {
        response.status = Some(self.status());
        response.body = Some(Box::new(serde_json::to_string(&self).unwrap()))
    }
}
