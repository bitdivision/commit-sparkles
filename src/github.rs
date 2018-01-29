use reqwest;
use reqwest::header::{Accept, UserAgent};
use reqwest::StatusCode;

use url::Url;

use errors::APIError;
use config::server_config::GithubConfig;

const GITHUB_HOST: &'static str = "https://github.com";

#[derive(Debug, Clone, Deserialize)]
pub struct GithubCredentials {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

impl GithubCredentials {
    pub fn request(github_constants: &GithubConfig, code: &str)
        -> Result<GithubCredentials, APIError> {

        // TODO Error Handling
        let endpoint = "/login/oauth/access_token";

        let url = format!("{}{}", GITHUB_HOST, endpoint);

        let mut request_url = Url::parse(&url)
            .expect("Could not parse Github oauth endpoint URL");

        request_url.query_pairs_mut()
                   .append_pair("client_id", &github_constants.client_id)
                   .append_pair("client_secret", &github_constants.client_secret)
                   .append_pair("code", &code);


        // TODO: move this into a persistent client pool?
        let client = reqwest::Client::new();

        let mut response = client.get(request_url)
                                 .header(Accept::json())
                                 .header(UserAgent::new(format!("CommitSparkles/{}", env!("CARGO_PKG_VERSION"))))
                                 .send()
                                 .unwrap();


        match response.status() {
            StatusCode::Ok=> (),
            _ => return Err(APIError::github_error())
        }

        let text_response = response.text().unwrap();

        // Github seems to return a 200 with error content if there's a problem
        // with one of the parameters. So check if the string contains 'error'
        // Horrible, but fine for the moment.
        if text_response.contains("error_uri") {
            return Err(APIError::github_error());
        }

        let credentials : GithubCredentials = match response.json() {
            Err(_) => return Err(APIError::github_error()),
            Ok(credentials) => credentials
        };

        println!("Credentials: {:?}", credentials);

        debug!("Github Credentials successfully deserialized");

        Ok(credentials)
    }
}
