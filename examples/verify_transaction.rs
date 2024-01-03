extern crate theteller;

use std::env;

use reqwest::Error;
use serde_json::Value;

use theteller::{client::r#async::Client, resources::transaction::Transaction};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let merchant_id = env::var("MERCHANT_ID").unwrap();

    let transaction_id = "123456789101".to_string();

    let teller_client = Client::create();

    let transaction = Transaction {
        transaction_id,
        merchant_id,
    };

    let transaction_response = transaction.verify(&teller_client).await?;
    let data: Value = transaction_response.json().await?;

    println!("{:?}", data);

    Ok(())
}
