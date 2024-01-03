extern crate theteller;

use std::env;

use reqwest::Error;
use serde_json::Value;

use theteller::{
    client::r#async::Client,
    constants::{account_issuers, processing_codes},
    resources::payments::{MomoDetails, Payment},
    utils::{amount_to_string::amount_to_string, generate_transaction_id::generate_transaction_id},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let merchant_id = env::var("MERCHANT_ID").unwrap();
    let subscriber_momo_number = env::var("TEST_MOMO_NUMBER").unwrap();

    let transaction_id = generate_transaction_id().to_string();
    let amount = amount_to_string(0.1);
    let momo_payment_processing_code = processing_codes::MOMO_PAYMENT.to_string();

    let teller_client = Client::create();

    let payment = Payment {
        merchant_id,
        transaction_id,
        processing_code: momo_payment_processing_code,
        amount,
        desc: "testing momo payment".to_string(),
        callback_url: "https://test.theteller.net/test".to_string(),
        reference: "Rust TheTeller SDK".to_string(),
        merchant_data: "test".to_string(),
    };

    let momo_details = MomoDetails {
        r_switch: account_issuers::VODAFONE.to_string(),
        subscriber_number: subscriber_momo_number,
    };

    let payment_response = payment.process_momo(&teller_client, &momo_details).await?;

    let data: Value = payment_response.json().await?;

    println!("{:?}", data);

    Ok(())
}
