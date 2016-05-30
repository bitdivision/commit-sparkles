extern crate iron;

extern crate router;

use iron::prelude::*;
use iron::status;

pub fn login_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello")))
}

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
    Ok(Response::with((status::Ok, "")))
}

