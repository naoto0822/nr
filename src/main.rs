use clap::Parser;
use std::process::Command;

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

#[derive(Debug)]
enum NotificationType {
    Error,
    Warning,
    Info,
    Success,
}

impl NotificationType {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "error" => Some(Self::Error),
            "warning" => Some(Self::Warning),
            "info" => Some(Self::Info),
            "success" => Some(Self::Success),
            _ => None,
        }
    }
    
    fn get_emoji(&self) -> &'static str {
        match self {
            Self::Error => "❌",
            Self::Warning => "⚠️",
            Self::Info => "ℹ️",
            Self::Success => "✅",
        }
    }
    
    fn get_default_title(&self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Warning => "Warning",
            Self::Info => "Information",
            Self::Success => "Success",
        }
    }
    
    fn get_sound(&self) -> &'static str {
        match self {
            Self::Error => "Sosumi",
            Self::Warning => "Funk",
            Self::Info => "Glass",
            Self::Success => "Hero",
        }
    }
}

fn main() {
    let args = Args::parse();
    
    let notification_type = args.r#type
        .as_ref()
        .and_then(|t| NotificationType::from_str(t));
    
    let title = match &args.title {
        Some(t) => t.clone(),
        None => match &notification_type {
            Some(nt) => format!("{} {}", nt.get_emoji(), nt.get_default_title()),
            None => "Notification".to_string(),
        }
    };
    
    let message = match &notification_type {
        Some(nt) => format!("{} {}", nt.get_emoji(), args.message),
        None => args.message,
    };
    
    send_notification(&title, &message, &notification_type, &args.url);
}

fn send_notification(title: &str, message: &str, notification_type: &Option<NotificationType>, url: &Option<String>) {
    let mut cmd = Command::new("terminal-notifier");
    cmd.arg("-title")
        .arg(title)
        .arg("-message")
        .arg(message);
    
    if let Some(nt) = notification_type {
        cmd.arg("-sound").arg(nt.get_sound());
    }
    
    if let Some(u) = url {
        cmd.arg("-open").arg(u);
    }
    
    let output = cmd.output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Notification sent successfully");
            } else {
                println!("Failed to send notification: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("Failed to execute terminal-notifier: {}", e);
            println!("Please check if terminal-notifier is installed: brew install terminal-notifier");
        }
    }
}