use theteller::{
    client::r#async::{self, Client},
    resources,
};

#[tokio::main]
async fn main() {
    let client = Client::new("hello");
}
