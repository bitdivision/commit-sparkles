use std::io::Read;
use std::env;

use hyper::client::Client;
use hyper::header::{Accept, qitem, UserAgent};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::status::StatusCode;

use serde_json;

use url::Url;

use errors::APIError;
use config::server_config::GithubConfig;

const GITHUB_HOST: &'static str = "https://github.com";

#[derive(Debug, Clone, Deserialize)]
pub struct GithubCredentials {
    access_token: String,
    scope: String,
    token_type: String,
}

impl GithubCredentials {
    pub fn request(github_constants: &GithubConfig, code: &str)
        -> Result<GithubCredentials, APIError> {

        // TODO Error Handling
        let endpoint = "/login/oauth/access_token";

        // TODO: move this into a persistent client pool?
        let client = Client::new();

        let url = format!("{}{}", GITHUB_HOST, endpoint);

        let mut request_url = Url::parse(&url)
            .expect("Could not parse Github oauth endpoint URL");

        request_url.query_pairs_mut()
                   .append_pair("client_id", &github_constants.client_id)
                   .append_pair("client_secret", &github_constants.client_secret)
                   .append_pair("code", &code);

        // Yikes! This seems rather excessive.
        // Accept: application/json
        let accept_header = Accept(vec![
                qitem(Mime(TopLevel::Application, SubLevel::Json,
                vec![])),
            ]);

        let user_agent = UserAgent(format!("CommitSparkles/{}", env!("CARGO_PKG_VERSION")));

        let mut response = client.get(request_url)
                            .header(accept_header)
                            .header(user_agent)
                            .send()
                            .unwrap();

        match(response.status) {
            StatusCode::Ok=> (),
            _ => return Err(APIError::github_error())
        }

        // TODO: Use content-length to set capacity
        let mut response_string = String::new();
        response.read_to_string(&mut response_string).unwrap();

        let credentials : GithubCredentials = match serde_json::from_str(&response_string) {
            Err(_) => return Err(APIError::github_error()),
            Ok(credentials) => credentials
        };

        println!("Credentials: {:?}", credentials);

        debug!("Github Credentials successfully deserialized");

        Ok(credentials)
    }
}
