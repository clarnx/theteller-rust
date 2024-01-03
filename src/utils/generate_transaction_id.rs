use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_transaction_id() -> String {
    // Get the current time as a Duration since the UNIX epoch
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Convert the Duration to milliseconds
    let current_time_stamp = duration.as_millis();

    // Generate a random number between 10000 and 99999
    let mut rng = rand::thread_rng();
    let random_component: u32 = rng.gen_range(10000..100000);

    // Combine timestamp and random component
    let combined_value = format!("{}{}", current_time_stamp.to_string(), random_component);

    // Take the last 12 characters as the unique ID
    let transaction_id = &combined_value[(combined_value.len().saturating_sub(12))..];

    return transaction_id.to_string();
}
