// Example of using the nr library from another Rust program
use nr::{Notify, Type};

fn main() {
    // Basic notification
    let notification = Notify::new("Hello from library!");
    if let Err(e) = notification.send() {
        eprintln!("Error: {}", e);
    }
    
    // Notification with all options
    let notification = Notify::new("Task completed successfully!")
        .with_title("Build Status")
        .with_type(Type::Success)
        .with_url("https://github.com");
    
    if let Err(e) = notification.send() {
        eprintln!("Error: {}", e);
    }
    
    // Error notification
    let notification = Notify::new("Something went wrong")
        .with_type(Type::Error);
    
    if let Err(e) = notification.send() {
        eprintln!("Error: {}", e);
    }
}