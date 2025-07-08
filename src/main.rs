use clap::Parser;
use nr::{Notification, NotificationType};

#[derive(Parser)]
#[command(name = "ccn")]
#[command(about = "Claude Code Notifier - macOS notification tool")]
struct Args {
    #[arg(long, help = "Notification title")]
    title: Option<String>,
    
    #[arg(long, help = "Notification message")]
    message: String,
    
    #[arg(long, help = "Notification type (error, warning, info, success)", value_name = "TYPE")]
    r#type: Option<String>,
    
    #[arg(long, help = "URL to open when notification is clicked")]
    url: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    let mut notification = Notification::new(&args.message);
    
    if let Some(title) = args.title {
        notification = notification.with_title(title);
    }
    
    if let Some(type_str) = args.r#type {
        if let Some(notification_type) = NotificationType::from_str(&type_str) {
            notification = notification.with_type(notification_type);
        }
    }
    
    if let Some(url) = args.url {
        notification = notification.with_url(url);
    }
    
    match notification.send() {
        Ok(()) => println!("Notification sent successfully"),
        Err(e) => eprintln!("{}", e),
    }
}