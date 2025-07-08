use std::process::Command;

#[derive(Debug, Clone)]
pub enum NotificationType {
    Error,
    Warning,
    Info,
    Success,
}

impl NotificationType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "error" => Some(Self::Error),
            "warning" => Some(Self::Warning),
            "info" => Some(Self::Info),
            "success" => Some(Self::Success),
            _ => None,
        }
    }
    
    pub fn get_emoji(&self) -> &'static str {
        match self {
            Self::Error => "❌",
            Self::Warning => "⚠️",
            Self::Info => "ℹ️",
            Self::Success => "✅",
        }
    }
    
    pub fn get_default_title(&self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Warning => "Warning",
            Self::Info => "Information",
            Self::Success => "Success",
        }
    }
    
    pub fn get_sound(&self) -> &'static str {
        match self {
            Self::Error => "Sosumi",
            Self::Warning => "Funk",
            Self::Info => "Glass",
            Self::Success => "Hero",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub title: String,
    pub message: String,
    pub notification_type: Option<NotificationType>,
    pub url: Option<String>,
}

impl Notification {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            title: "Notification".to_string(),
            message: message.into(),
            notification_type: None,
            url: None,
        }
    }
    
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
    
    pub fn with_type(mut self, notification_type: NotificationType) -> Self {
        self.notification_type = Some(notification_type);
        self
    }
    
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    
    pub fn send(&self) -> Result<(), NotificationError> {
        send_notification(self)
    }
}

#[derive(Debug)]
pub enum NotificationError {
    ExecutionFailed(String),
    CommandNotFound(String),
}

impl std::fmt::Display for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationError::ExecutionFailed(msg) => write!(f, "Failed to send notification: {}", msg),
            NotificationError::CommandNotFound(msg) => write!(f, "Command not found: {}", msg),
        }
    }
}

impl std::error::Error for NotificationError {}

pub fn send_notification(notification: &Notification) -> Result<(), NotificationError> {
    let title = if notification.title.is_empty() {
        match &notification.notification_type {
            Some(nt) => format!("{} {}", nt.get_emoji(), nt.get_default_title()),
            None => "Notification".to_string(),
        }
    } else {
        notification.title.clone()
    };
    
    let message = match &notification.notification_type {
        Some(nt) => format!("{} {}", nt.get_emoji(), notification.message),
        None => notification.message.clone(),
    };
    
    let mut cmd = Command::new("terminal-notifier");
    cmd.arg("-title")
        .arg(&title)
        .arg("-message")
        .arg(&message);
    
    if let Some(nt) = &notification.notification_type {
        cmd.arg("-sound").arg(nt.get_sound());
    }
    
    if let Some(url) = &notification.url {
        cmd.arg("-open").arg(url);
    }
    
    let output = cmd.output()
        .map_err(|e| NotificationError::CommandNotFound(format!("terminal-notifier: {}. Please install with: brew install terminal-notifier", e)))?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(NotificationError::ExecutionFailed(String::from_utf8_lossy(&output.stderr).to_string()))
    }
}