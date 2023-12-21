use std::collections::HashMap;

use base64::{engine::general_purpose, Engine};
use reqwest::{Client as ReqwestClient, RequestBuilder};

pub struct Client<'a> {
    url: &'a str,
    request_headers: HashMap<&'a str, &'a str>,
    request_payload: HashMap<&'a str, &'a str>,
}

impl<'a> Client <'a>{
    /// Create a new account with the given secret key.
    pub fn new(secret_key: &str) -> Self {


        let base_64_encoded_secret = general_purpose::STANDARD.encode(secret_key);
        let authorization = format!("Basic {}", base_64_encoded_secret);

        let mut request_headers = HashMap::new();
        

        request_headers.insert("Content-Type", "application/json");
        request_headers.insert("Authorization", authorization.as_str());
        request_headers.insert("Cache-Control", "no-cache");

        Self {
            url: "",
            request_headers: HashMap::new(),
            request_payload: HashMap::new()
        }

    }

    pub fn configure(&mut self, url: &'a str, request_payload: HashMap<&'a str, &'a str>) {
        let client = ReqwestClient::new();

        self.url = url;
        self.request_payload = request_payload;

        // client.

    }

    // pub fn create_request(&mut self, method: RequestMethod, url: &str) -> RequestBuilder {
    //     self.url = url.to_string();

    //     let client = ReqwestClient::new();

    //     match method {
    //         GET => client.request(reqwest::Method::GET, url),
    //         POST => client.request(reqwest::Method::POST, url),
    //         PUT => client.request(reqwest::Method::PUT, url),
    //         PATCH => client.request(reqwest::Method::PATCH, url),
    //         DELETE => client.request(reqwest::Method::DELETE, url),
    //     }
    // }
}
