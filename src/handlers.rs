extern crate iron;

extern crate router;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;

use data::{GetToken};


/// Called by the front-end after a successful OAuth redirect.
///
/// Parameters:
///     `code`: The code provided by OAuth
///     `state`: The random string provided when initialising
///
/// This endpoint will pass the provided code to the following
/// Github endpoint:
///
/// `https://github.com/login/oauth/access_token`
///
/// With the following params:
///     `client_id`
///     `client_secret`
///     `code`
///     `state`
///
/// The request will be made with an `Accept: application/json`
/// header.
///
/// The response will include an `access_token`, `scope` and
/// `token_type`
///
/// This will be used to create the User in the database and a
/// JWT will be returned to allow future authorization.
///
pub fn oauth_get_token(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Struct<GetToken>>();
    match body {
        Ok(Some(body)) => info!("Parsed to {:?}", body),
        Ok(None) => error!("No body"),
        Err(err) => error!("Error: {:?}", err)
    }
    Ok(Response::with((status::Ok, "")))
}

