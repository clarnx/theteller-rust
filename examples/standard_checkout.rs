extern crate theteller;

use std::env;

use reqwest::Error;
use serde_json::Value;

use theteller::{
    client::r#async::Client,
    resources::checkout::Checkout,
    utils::{
        amount_to_minor_unit::amount_to_minor_unit,
        generate_transaction_id::generate_transaction_id,
    },
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api_secret_binding = env::var("API_SECRET").unwrap();
    let api_secret = api_secret_binding.as_str();

    let merchant_id = env::var("MERCHANT_ID").unwrap();

    let transaction_id = generate_transaction_id().to_string();
    let amount = amount_to_minor_unit(0.12);

    let teller_client = Client::new(api_secret);

    let checkout = Checkout {
        merchant_id,
        amount,
        email: "test@example.com".to_string(),
        redirect_url: "https://google.com".to_string(),
        transaction_id,
        desc: "testing".to_string(),
    };

    let checkout_response = checkout.initiate(&teller_client).await?;

    let data: Value = checkout_response.json().await?;

    println!("{:?}", data);

    Ok(())
}
