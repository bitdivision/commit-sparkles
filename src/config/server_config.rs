use std::io::Read;
use std::path::Path;
use std::fs::File;

use iron::typemap::Key;

use toml;

use errors::ConfigError;

// TODO: Add option type to this to allow optional values.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub environment: EnvironmentConfig,
    pub github: GithubConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvironmentConfig {
    pub environment_name: String,
    pub log_config: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GithubConfig{
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub root_url: String,
    pub host_ip: String,
    pub host_port: u16,
    pub pg_connection_string: String,
}


impl Config {

    pub fn new(config_file_path: &Path) -> Result<Config, ConfigError> {
        let toml_value = load_file(config_file_path)?;

        let config: Config = toml_value.try_into()?;
        Ok(config)
    }
}

fn load_file(toml_file_path: &Path) -> Result<toml::Value, ConfigError> {
    let mut toml_file = File::open(toml_file_path)?;
    let mut config_contents = String::new();
    toml_file.read_to_string(&mut config_contents)?;
    let t = config_contents.parse()?;
    Ok(t)
    //None => Err(ConfigError::ParseError(parser.errors.pop().unwrap())),
}

impl Key for Config {
    type Value = Config;
}
