use serde::Serialize;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Checkout {
    merchant_id: &'static str,
    transaction_id: &'static str,
    desc: &'static str,
    amount: &'static str,
    redirect_url: &'static str,
    email: &'static str,
    api_key: &'static str,
    apiuser: &'static str,
}

impl Checkout {}
