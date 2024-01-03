use std::env;

use base64::{engine::general_purpose, Engine};
use reqwest::{header, Client, Error, Response};
use serde::Serialize;
use serde_json::{json, Value};

use crate::{
    constants::{
        account_issuers,
        endpoints::{PAYMENT_ENDPOINT_ASYNC, TRANSFERS_ENDPOINT},
    },
    traits::custom_client_functions::CustomClientFunctions,
    utils::generate_request_id::generate_request_id,
};

#[derive(Debug, Serialize, Default)]
pub struct MomoDetails {
    pub r_switch: String,
    pub subscriber_number: String,
}

#[derive(Debug, Serialize, Default)]
pub struct Payment {
    pub merchant_id: String,
    pub transaction_id: String,
    pub amount: String,
    pub processing_code: String,
    pub desc: String,
    pub callback_url: String,
    pub reference: String,
    pub merchant_data: String,
}

impl Payment {
    fn new() -> Self {
        Self {
            merchant_id: "".to_string(),
            transaction_id: "".to_string(),
            amount: "".to_string(),
            processing_code: "".to_string(),
            desc: "".to_string(),
            callback_url: "".to_string(),
            reference: "".to_string(),
            merchant_data: "".to_string(),
        }
    }

    pub async fn process_momo(
        &self,
        client: &Client,
        momo_details: &MomoDetails,
    ) -> Result<Response, Error> {
        let api_secret_binding = env::var("API_SECRET").unwrap();
        let api_secret = api_secret_binding.as_str();

        let base_64_encoded_secret = general_purpose::STANDARD.encode(api_secret);
        let authorization = format!("Basic {}", base_64_encoded_secret);
        let generated_request_id = generate_request_id();

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

        // Use from_str instead of from_static for the Authorization header
        if let Ok(request_id) = header::HeaderValue::from_str(&generated_request_id) {
            request_headers.insert("request-id", request_id);
        } else {
            // Handle the error if necessary
            eprintln!("Failed to create request id header");
        }

        request_headers.insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("no-cache"),
        );

        // Convert the struct to a JSON string
        let json_string = serde_json::to_string(&self).unwrap();

        // Parse the JSON string into a serde_json::Value
        let mut json_value: Value = serde_json::from_str(&json_string).unwrap();

        if let Some(obj) = json_value.as_object_mut() {
            obj.insert(
                "subscriber_number".to_string(),
                json!(momo_details.subscriber_number),
            );
            obj.insert("r-switch".to_string(), json!(momo_details.r_switch));
        }

        let json_string_including_momo_details = serde_json::to_string(&json_value).unwrap();

        let json_value_including_momo_details: Value =
            serde_json::from_str(&json_string_including_momo_details).unwrap();

        let response = client
            .set_headers(request_headers)
            .post(format!("{}", PAYMENT_ENDPOINT_ASYNC))
            .json(&json_value_including_momo_details)
            .send()
            .await;

        response
    }

    // TODO: Add card details
    pub async fn process_bank_acc(
        &self,
        client: &Client,
        bank_code: &str,
    ) -> Result<Response, Error> {
        // Convert the struct to a JSON string
        let json_string = serde_json::to_string(&self).unwrap();

        // Parse the JSON string into a serde_json::Value
        let mut json_value: Value = serde_json::from_str(&json_string).unwrap();

        if let Some(obj) = json_value.as_object_mut() {
            obj.insert(
                "account_issuer".to_string(),
                json!(account_issuers::BANK_TRANSFER),
            );
            obj.insert("account_bank".to_string(), json!(bank_code));
        }

        let json_string_including_account_bank = serde_json::to_string(&json_value).unwrap();

        let json_value_including_account_bank: Value =
            serde_json::from_str(&json_string_including_account_bank).unwrap();

        let response = client
            .post(format!("{}/process", TRANSFERS_ENDPOINT))
            .json(&json_value_including_account_bank)
            .send()
            .await;

        response
    }
}
