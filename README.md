# nr

A macOS notification library and CLI tool that provides a simple interface to send notifications using terminal-notifier.

## Features

- 🎯 Simple and intuitive API
- 🔧 Both CLI tool and Rust library
- ✨ Support for different notification types (error, warning, info, success)
- 🔗 Clickable notifications with URL support
- 🎵 Custom sounds for different notification types
- 📦 Zero external dependencies (except terminal-notifier)

## Prerequisites

This tool requires `terminal-notifier` to be installed on your macOS system:

```bash
brew install terminal-notifier
```

### System Configuration
To use terminal-notifier, you need to configure macOS notification settings:

1. Open System Preferences → Notifications
2. Find and select terminal-notifier
3. Check "Allow Notifications"
4. Set notification style (Banner/Alert) as needed

## Installation

### As a CLI tool

```bash
cargo install nr
```

### As a library dependency

Add this to your `Cargo.toml`:

```toml
[dependencies]
nr = "0.1.0"
```

## CLI Usage

```bash
# Basic notification
nr --message "Hello, World!"

# Notification with title and type
nr --title "Build Status" --message "Build completed successfully" --type success

# Notification with clickable URL
nr --message "Check the results" --url "https://github.com"

# Error notification
nr --message "Something went wrong" --type error
```

### CLI Options

- `--title <TITLE>`: Notification title
- `--message <MESSAGE>`: Notification message (required)
- `--type <TYPE>`: Notification type (error, warning, info, success)
- `--url <URL>`: URL to open when notification is clicked

## Library Usage

```rust
use nr::{Notify, Type};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic notification
    Notify::new("Hello from Rust!")
        .send()?;
    
    // Notification with all options
    Notify::new("Task completed successfully!")
        .with_title("Build Status")
        .with_type(Type::Success)
        .with_url("https://github.com")
        .send()?;
    
    // Error notification
    Notify::new("Something went wrong")
        .with_type(Type::Error)
        .send()?;
    
    Ok(())
}
```

### Available Notification Types

- `Type::Error` - ❌ with "Sosumi" sound
- `Type::Warning` - ⚠️ with "Funk" sound  
- `Type::Info` - ℹ️ with "Glass" sound
- `Type::Success` - ✅ with "Hero" sound

## API Documentation

### `Notify`

The main struct for creating and sending notifications.

#### Methods

- `new(message: impl Into<String>) -> Self` - Create a new notification with a message
- `with_title(self, title: impl Into<String>) -> Self` - Set the notification title
- `with_type(self, notification_type: Type) -> Self` - Set the notification type
- `with_url(self, url: impl Into<String>) -> Self` - Set a URL to open when clicked
- `send(&self) -> Result<(), NrError>` - Send the notification

### `Type`

Enum representing different types of notifications with associated emojis and sounds.

### Error Handling

The library uses `NrError` enum for error handling:

- `NrError::ExecutionFailed` - terminal-notifier execution failed
- `NrError::CommandNotFound` - terminal-notifier not found on system

## Examples

See the `examples/` directory for more usage examples.

## License

MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.