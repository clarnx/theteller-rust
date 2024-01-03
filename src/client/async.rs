use base64::{engine::general_purpose, Engine};
use reqwest::{
    header::{self, HeaderMap},
    Client as ReqwestClient,
};

use crate::traits::custom_client_functions::CustomClientFunctions;

pub struct Client;

impl CustomClientFunctions for ReqwestClient {
    fn set_headers(&self, request_headers: HeaderMap) -> ReqwestClient {
        // Create a builder for the client
        let client_builder = ReqwestClient::builder();

        // Add headers to the builder
        let configured_client_builder = client_builder.default_headers(request_headers);

        // Build the client
        let client = configured_client_builder.build().unwrap();

        client
    }
}

impl<'a> Client {
    /// Create a new account with the given secret key.
    pub fn new(secret_key: &'a str) -> ReqwestClient {
        let base_64_encoded_secret = general_purpose::STANDARD.encode(secret_key);
        let authorization = format!("Basic {}", base_64_encoded_secret);

        let mut request_headers = header::HeaderMap::new();
        request_headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        // Use from_str instead of from_static for the Authorization header
        if let Ok(authorization_header) = header::HeaderValue::from_str(&authorization) {
            request_headers.insert(header::AUTHORIZATION, authorization_header);
        } else {
            // Handle the error if necessary
            eprintln!("Failed to create Authorization header");
        }

        request_headers.insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("no-cache"),
        );

        return Self::configure(request_headers);
    }

    pub fn create() -> ReqwestClient {
        ReqwestClient::new()
    }

    fn configure(request_headers: HeaderMap) -> ReqwestClient {
        // Create a builder for the client
        let client_builder = ReqwestClient::builder();

        // Add headers to the builder
        let configured_client_builder = client_builder.default_headers(request_headers);

        // Build the client
        let client = configured_client_builder.build().unwrap();

        client
    }

    // pub fn set_headers(&self, request_headers: HeaderMap) -> ReqwestClient {
    //     return Self::configure(request_headers);
    // }

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
