fn main() {
    loop {
        // Get current time in milliseconds
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        // Busy wait for 300 milliseconds
        while std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            < now + 300
        {}

        // Sleep for 10 seconds
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
