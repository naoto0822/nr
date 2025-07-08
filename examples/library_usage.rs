// Example of using the nr library from another Rust program
use nr::{Notification, NotificationType};

fn main() {
    // Basic notification
    let notification = Notification::new("Hello from library!");
    if let Err(e) = notification.send() {
        eprintln!("Error: {}", e);
    }
    
    // Notification with all options
    let notification = Notification::new("Task completed successfully!")
        .with_title("Build Status")
        .with_type(NotificationType::Success)
        .with_url("https://github.com");
    
    if let Err(e) = notification.send() {
        eprintln!("Error: {}", e);
    }
    
    // Error notification
    let notification = Notification::new("Something went wrong")
        .with_type(NotificationType::Error);
    
    if let Err(e) = notification.send() {
        eprintln!("Error: {}", e);
    }
}