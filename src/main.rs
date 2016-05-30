#![feature(question_mark)]

extern crate iron;
extern crate mount;
#[macro_use] extern crate router;

#[macro_use] extern crate log;
extern crate log4rs;
extern crate logger;

extern crate toml;
extern crate rustc_serialize;
extern crate docopt;

use std::path::Path;

use iron::prelude::*;
use mount::Mount;

use docopt::Docopt;

use config::server_config;

use logger::Logger;

mod handlers;
mod config;

const USAGE: &'static str = "
Commit Sparkles Server

Usage:
    commitsparkles (-h | --help)
    commitsparkles --version
    commitsparkles [--config=<toml_config>]

Options:
    -h --help                   Show this screen.
    --version                   Show version.
    --config=<toml_config>      Use a TOML config file. [default: config/dev.toml]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_config: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());

    let config = server_config::Config::new(Path::new(&args.flag_config))
        .expect("Failed to load configuration!");

    log4rs::init_file(config.environment.log_config, Default::default())
        .expect("Log initialisation failed!");

    info!("Loaded configuration for environment {:?}", config.environment.environment_name);

    let mut mount = Mount::new();

    mount.mount("user", router!(
        get "/login" => handlers::login_handler,
        get "/oauth/oauth_get_token" => handlers::oauth_get_token,
    ));

    let mut chain = Chain::new(mount);

    // Set up the logger middleware
    // This logs requests and responses to the console.
    // TODO: Write a more standardised logger to use log macros.
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    info!("Initializing Server on: {}:{}", config.server.host_ip, config.server.host_port);

    // Need a &str to satisfy ToSocketAddrs. Passing &host_ip doesn't work unless hinted.
    // TODO: Shouldn't auto deref take care of this?
    let socket_addr: (&str, u16) = (&config.server.host_ip, config.server.host_port);

    Iron::new(chain).http(socket_addr).unwrap();

}
