pub mod price;
pub mod strategy;
pub mod tokens;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Spawn a loading animation thread that runs until stopped
pub fn spawn_loading_animation(message: &str, max_duration_ms: u64) -> Arc<Mutex<bool>> {
    let should_stop = Arc::new(Mutex::new(false));
    let should_stop_clone = Arc::clone(&should_stop);
    let message = message.to_string();
    
    thread::spawn(move || {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut frame_index = 0;
        let start_time = std::time::Instant::now();
        
        loop {
            // Check if we should stop
            {
                let stop_guard = should_stop_clone.lock().unwrap();
                if *stop_guard {
                    break;
                }
            }
            
            // Check if we've exceeded max duration
            if start_time.elapsed().as_millis() >= max_duration_ms as u128 {
                break;
            }
            
            print!("\r{} {} ", frames[frame_index], message);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(100));
            frame_index = (frame_index + 1) % frames.len();
        }
        
        // Clear the line and show completion
        println!("\r✓ {} completed", message);
    });
    
    should_stop
}

/// Print a log message. Only prints if the environment variable `ENV` is set to `development`.
pub fn print_log(message: &str) {
    if std::env::var("ENV").unwrap_or_default() == "development" {
        println!("[Trading Agent] {}", message);
    }
}