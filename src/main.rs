#![feature(question_mark)]
#![feature(rustc_private)]

extern crate iron;
extern crate mount;


#[macro_use]
extern crate log;
extern crate log4rs;

extern crate toml;

extern crate rustc_serialize;

#[macro_use] extern crate router;


use iron::prelude::*;
use std::path::Path;

use mount::Mount;

mod handlers;
mod config;

use config::serverConfig;


fn main() {
    match log4rs::init_file("config/logging.toml", Default::default()) {
        Ok(_) => (),
        Err(e) => panic!("Log initialisation failed! {:?}",e),
    };

    let server_config = match serverConfig::Config::new(
        Path::new("config/dev.toml"))
    {
        Ok(_) => (),
        Err(e) => panic!("Failed to load configuration. Error {:?}", e)
    };

    info!("Initializing Booster Engines");

    let mut mount = Mount::new();

    mount.mount("user", router!(
        get "/login" => handlers::login_handler,
    ));

    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}
