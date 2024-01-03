use reqwest::{Client, Error, Response};
use serde::Serialize;

use crate::constants::endpoints::CHECKOUT_ENDPOINT;

#[derive(Debug, Serialize, Default)]
pub struct Checkout {
    pub merchant_id: String,
    pub transaction_id: String,
    pub desc: String,
    pub amount: String,
    pub redirect_url: String,
    pub email: String,
}

impl Checkout {
    fn new() -> Self {
        Self {
            merchant_id: "".to_string(),
            transaction_id: "".to_string(),
            desc: "".to_string(),
            amount: "".to_string(),
            redirect_url: "".to_string(),
            email: "".to_string(),
        }
    }

    pub async fn initiate(&self, client: &Client) -> Result<Response, Error> {
        let response = client
            .post(format!("{}/initiate", CHECKOUT_ENDPOINT))
            .json(self)
            .send()
            .await;

        response
    }
}
