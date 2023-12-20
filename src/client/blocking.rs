use std::collections::HashMap;

use base64::encode;
use reqwest::{Client, RequestBuilder};

#[derive(Debug, Clone, Copy)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

use RequestMethod::*;

pub struct TheTellerClient {
    url: String,
    request_headers: HashMap<&'static str, String>,
    request_payload: HashMap<&'static str, String>,
}

impl TheTellerClient {
    /// Create a new account with the given secret key.
    pub fn new(secret_key: &str) -> Self {
        let base_64_encoded_secret = encode(secret_key);

        let mut request_headers = HashMap::new();
        let mut request_payload = HashMap::new();

        request_headers.insert("Content-Type", "application/json".to_string());
        request_headers.insert("Authorization", format!("Basic {}", base_64_encoded_secret));
        request_headers.insert("Cache-Control", "no-cache".to_string());

        Self {
            url: "".to_string(),
            request_headers,
            request_payload,
        }
    }

    pub fn create_request(&mut self, method: RequestMethod, url: &str) -> RequestBuilder {
        self.url = url.to_string();

        let client = Client::new();

        match method {
            GET => client.request(reqwest::Method::GET, url),
            POST => client.request(reqwest::Method::POST, url),
            PUT => client.request(reqwest::Method::PUT, url),
            PATCH => client.request(reqwest::Method::PATCH, url),
            DELETE => client.request(reqwest::Method::DELETE, url),
        }
    }
}