use reqwest::{Client, Error, Response};
use serde::Serialize;
use serde_json::{json, Value};

use crate::constants::{account_issuers, endpoints::TRANSFERS_ENDPOINT};

#[derive(Debug, Serialize, Default)]
pub struct Transfer {
    pub account_number: String,
    pub merchant_id: String,
    pub transaction_id: String,
    pub processing_code: String,
    pub amount: String,

    #[serde(rename = "r-switch")]
    pub r_switch: String,

    pub desc: String,
    pub pass_code: String,
}

impl Transfer {
    fn new() -> Self {
        Self {
            account_number: "".to_string(),
            merchant_id: "".to_string(),
            transaction_id: "".to_string(),
            processing_code: "".to_string(),
            amount: "".to_string(),
            r_switch: "FLT".to_string(),
            desc: "".to_string(),
            pass_code: "".to_string(),
        }
    }

    // pub async fn process_momo(
    //     &self,
    //     client: &Client,
    //     momo_network: &str,
    // ) -> Result<Response, Error> {
    //     let response = client
    //         .post(format!("{}/process", TRANSFERS_ENDPOINT))
    //         .json(self)
    //         .send()
    //         .await;

    //     response
    // }
    pub async fn process_momo(
        &self,
        client: &Client,
        momo_network: &str,
    ) -> Result<Response, Error> {
        // Convert the struct to a JSON string
        let json_string = serde_json::to_string(&self).unwrap();

        // Parse the JSON string into a serde_json::Value
        let mut json_value: Value = serde_json::from_str(&json_string).unwrap();

        if let Some(obj) = json_value.as_object_mut() {
            obj.insert("account_issuer".to_string(), json!(momo_network));
        }

        let json_string_including_account_issuer = serde_json::to_string(&json_value).unwrap();

        let json_value_including_account_issuer: Value =
            serde_json::from_str(&json_string_including_account_issuer).unwrap();

        let response = client
            .post(format!("{}/process", TRANSFERS_ENDPOINT))
            .json(&json_value_including_account_issuer)
            .send()
            .await;

        response
    }

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
