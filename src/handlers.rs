extern crate iron;

extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

pub fn login_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello")))
}
