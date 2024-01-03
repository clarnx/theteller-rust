extern crate theteller;

use std::env;

use reqwest::Error;
use serde_json::Value;

use theteller::{
    client::r#async::Client,
    constants::{bank_codes, processing_codes},
    resources::transfer::Transfer,
    utils::{
        amount_to_minor_unit::amount_to_minor_unit,
        generate_transaction_id::generate_transaction_id,
    },
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_secret_binding = env::var("API_SECRET").unwrap();
    let api_secret = api_secret_binding.as_str();

    let merchant_api_key = env::var("MERCHANT_API_KEY").unwrap();
    let merchant_pass_code = env::var("MERCHANT_PASS_CODE").unwrap();
    let receiver_bank_account = env::var("TEST_BANK_ACCOUNT").unwrap();

    let transaction_id = generate_transaction_id().to_string();
    let amount = amount_to_minor_unit(0.12);

    let bank_transfer_processing_code = processing_codes::BANK_TRANSFER.to_string();

    let teller_client = Client::new(api_secret);

    let transfer = Transfer {
        account_number: receiver_bank_account,
        merchant_id: merchant_api_key,
        transaction_id,
        processing_code: bank_transfer_processing_code,
        amount,
        r_switch: "FLT".to_string(),
        desc: "testing bank transfer".to_string(),
        pass_code: merchant_pass_code,
    };

    let transfer_response = transfer
        .process_bank_acc(&teller_client, bank_codes::ECOBANK_GHANA_LTD)
        .await?;

    let data: Value = transfer_response.json().await?;

    println!("{:?}", data);

    Ok(())
}
