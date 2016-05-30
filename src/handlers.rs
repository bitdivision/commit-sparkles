extern crate iron;

extern crate router;
extern crate bodyparser;
extern crate hyper;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use hyper::client::Client;

use persistent::Read as PersistentRead;

use data::{GetToken};
use errors::APIError;
use config::server_config::Config;

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
    let body = match req.get::<bodyparser::Struct<GetToken>>() {
        Ok(Some(body)) => {
            trace!("Decoded body to: {:?}", body);
            body
        },
        Ok(None) => {
            let error = APIError::no_body();
            return Err(IronError::new(error.clone(), error))
        }
        Err(err) => {
            let error = APIError::bad_json();
            return Err(IronError::new(error.clone(), error))
        }
    };

    //HACK! Replace soon. Promise
    let client = Client::new();
    let config = req.get::<PersistentRead<Config>>().unwrap();
    let mut res = client.get(&format!("https://github.com/login/oauth/access_token?client_id={}&client_secret={}&code={}", &config.github.client_id, &config.github.client_secret, &body.code)).send().unwrap();
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("Received response: {:?}", s);

    Ok(Response::with((status::Ok, "")))
}

