#![feature(question_mark)]

extern crate iron;
extern crate mount;
#[macro_use] extern crate router;

#[macro_use] extern crate log;
extern crate log4rs;

extern crate toml;
extern crate rustc_serialize;
extern crate docopt;

use std::path::Path;

use iron::prelude::*;
use mount::Mount;

use docopt::Docopt;

use config::serverConfig;

mod handlers;
mod config;

const USAGE: &'static str = "
Commit Sparkles Server

Usage:
    commitsparkles (-h | --help)
    commitsparkles --version
    commitsparkles --config=<toml_config>

Options:
    -h --help                   Show this screen.
    --version                   Show version.
    --config=<toml_config>      Use a TOML config file.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_config: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());

    let config = serverConfig::Config::new(Path::new(&args.flag_config))
        .expect("Failed to load configuration!");

    log4rs::init_file(config.environment.log_config, Default::default())
        .expect("Log initialisation failed!");

    info!("Loaded configuration for environment {:?}", config.environment.environment_name);

    let mut mount = Mount::new();

    mount.mount("user", router!(
        get "/login" => handlers::login_handler,
    ));

    Iron::new(mount).http("0.0.0.0:3000").unwrap();

    info!("Server initialised");
}
