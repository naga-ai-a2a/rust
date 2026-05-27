fn main() {
    // Using SystemTime
    let now = std::time::SystemTime::now();
    println!("\n . now: {:?}", now);

    let duration_since_epoch = now.duration_since(
        std::time::UNIX_EPOCH).unwrap();

    println!("\n . duration_since_epoch: {:?}", duration_since_epoch);
    
    // Displaying the duration in a human-friendly format
    let human_friendly_duration = humantime::format_duration(duration_since_epoch).to_string();
    println!("\n . Current time since UNIX epoch: {}", human_friendly_duration);

    let now = chrono::Utc::now();

    let ts = now.timestamp();
    println!("\n . Unix Timestamp: {}", ts);

    println!("\n . Milliseconds: {}", now.timestamp_millis());

    if let Some(dt) = chrono::DateTime::from_timestamp(ts, 0) {
        println!("\n . The date is: {}", dt);
    }
}
