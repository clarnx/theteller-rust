use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_request_id() -> String {
    // Get the current time as a Duration since the UNIX epoch
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    // Convert the Duration to milliseconds
    let current_time_stamp = duration.as_millis();

    // Generate a random number between 10000 and 99999
    let mut rng = rand::thread_rng();
    let random_component: u32 = rng.gen_range(100..1000);

    // Combine timestamp and random component into 16 characters
    let combined_value = format!("{}{}", current_time_stamp.to_string(), random_component);

    return combined_value;
}
