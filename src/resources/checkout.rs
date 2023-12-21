use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct CheckoutPayload {
    merchant_id: &'static str,
    transaction_id: &'static str,
    desc: &'static str,
    amount: &'static str,
    redirect_url: &'static str,
    email: &'static str,
    api_key: &'static str,
    apiuser: &'static str,
}

impl CheckoutPayload {}

pub struct Checkout;

impl Checkout {
    fn initiate(url: &str, request_payload: CheckoutPayload) {
        // let request_client = clien;
    }
}
