extern crate iron;
extern crate mount;

extern crate log4rs;

#[macro_use]
extern crate router;


use iron::prelude::*;
use iron::status;

use mount::Mount;

mod handlers;

fn main() {

    match log4rs::init_file("config/log.toml", Default::default()) {
        Ok(_) => (),
        Err(e) => panic!("Log initialisation failed! {:?}",e),
    }

    let mut mount = Mount::new();

    mount.mount("user", router!(
        get "/login" => handlers::login_handler,
    ));

    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}
