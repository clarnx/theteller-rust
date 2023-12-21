use reqwest::Client;
use serde::Serialize;

use crate::traits::create_request_payload::CreateRequestPayload;

#[derive(Debug, Serialize, Default)]
pub struct CheckoutPayload {
    merchant_id: &'static str,
    transaction_id: &'static str,
    desc: &'static str,
    amount: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    redirect_url: &'static str,
    email: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    api_key: &'static str,
    #[serde(skip_serializing_if = "str::is_empty")]
    apiuser: &'static str,
}

impl CheckoutPayload {}

impl CreateRequestPayload for CheckoutPayload {
    fn new() -> Self {
        Self {
            merchant_id: "",
            transaction_id: "",
            desc: "",
            amount: "",
            redirect_url: "",
            email: "",
            api_key: "",
            apiuser: "",
        }
    }
}

pub struct Checkout;

impl Checkout {
    fn initiate(client: &Client, request_payload: impl CreateRequestPayload) {
        // client.
    }
}
