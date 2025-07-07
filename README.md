# nr (Notification Rust)

A macOS notification tool that sends system notifications using terminal-notifier.

## Installation

### Prerequisites
```bash
brew install terminal-notifier
```

### System Configuration
To use terminal-notifier, you need to configure macOS notification settings:

1. Open System Preferences → Notifications
2. Find and select terminal-notifier
3. Check "Allow Notifications"
4. Set notification style (Banner/Alert) as needed

## Usage

### Basic notification
```bash
nr --message "Task completed"
```

### Notification with title
```bash
nr --title "Build Complete" --message "Compilation finished successfully"
```

### Notification with type
```bash
# Error notification
nr --message "Build error occurred" --type error

# Warning notification
nr --message "Tests failed" --type warning

# Info notification
nr --message "Build started" --type info

# Success notification
nr --message "All tests completed" --type success
```

## Options

- `--title`: Notification title (auto-generated based on type if omitted)
- `--message`: Notification message (required)
- `--type`: Notification type (error, warning, info, success)

When a notification type is specified, appropriate emoji, sound, and default title are automatically set.