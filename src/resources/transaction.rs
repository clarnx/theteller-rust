use reqwest::{header, Client, Error, Response};
use serde::Serialize;

use crate::{
    constants::endpoints::TRANSACTIONS_ENDPOINT,
    traits::custom_client_functions::CustomClientFunctions,
};

#[derive(Debug, Serialize, Default)]
pub struct Transaction {
    pub transaction_id: String,
    pub merchant_id: String,
}

impl Transaction {
    fn new() -> Self {
        Self {
            transaction_id: "".to_string(),
            merchant_id: "".to_string(),
        }
    }

    pub async fn verify(&self, client: &Client) -> Result<Response, Error> {
        let merchant_id: &str = self.merchant_id.as_str();
        let mut request_headers = header::HeaderMap::new();

        if let Ok(merchant_id) = header::HeaderValue::from_str(&merchant_id) {
            request_headers.insert("Merchant-Id", merchant_id);
        } else {
            // Handle the error if necessary
            eprintln!("Failed to create Merchant ID header");
        }

        request_headers.insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("no-cache"),
        );

        let response = client
            .set_headers(request_headers)
            .get(format!(
                "{}/{}/status",
                TRANSACTIONS_ENDPOINT, self.transaction_id
            ))
            .send()
            .await;

        response
    }
}
