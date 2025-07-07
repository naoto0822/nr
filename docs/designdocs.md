# CCN Design Document

## Project Overview

CCN (Claude Code Notifier) is a tool that sends macOS notifications when work is completed in Claude Code.

## Purpose

To improve work efficiency by allowing users to receive real-time notifications when tasks are completed in Claude Code.

## Functional Requirements

### Basic Features
- Receive work completion notifications from Claude Code
- Display native macOS notifications
- Customize notification content

### Notification Types
- Task completion notifications
- Error occurrence notifications
- Approval waiting notifications

## Technical Approach

### Technologies Used
- **Language**: Rust
- **Notification System**: `terminal-notifier`

### Prerequisites
- `terminal-notifier` installation required
  ```bash
  brew install terminal-notifier
  ```
- **Important**: To use terminal-notifier, notification permission is required in Mac system preferences
  1. Open System Preferences → Notifications
  2. Find and select terminal-notifier
  3. Check "Allow Notifications"
  4. Set notification style (Banner/Alert) as needed

### Architecture
```
Claude Code hooks → CCN Binary → macOS Notifications
```

### Operation Method
CCN is provided as a binary and sends notifications triggered by Claude Code's hooks feature.

## Implementation Plan

### Phase 1: Basic Notification Features
- Basic implementation of macOS notifications
- Specify notification content via command line arguments

### Phase 2: Claude Code Integration
- Integration features with Claude Code
- Automatic notifications using hooks

## Usage Examples

```bash
# Basic notification
ccn --title "Work Complete" --message "Code generation by Claude Code completed"

# Error notification
ccn --title "Error Occurred" --message "Build error occurred" --type error

# Warning notification
ccn --title "Warning" --message "Tests failed" --type warning

# Info notification
ccn --title "Information" --message "Build started" --type info
```

## Utilizing Notification Types (--type)

By using the `--type` option, the following features can be achieved:

### 1. Automatic Title Setting
- **error**: "Error"
- **warning**: "Warning"
- **info**: "Information"
- **success**: "Success"

### 2. Audio Notification Differentiation
- **error**: High urgency alert sound
- **warning**: Attention-getting sound
- **info**: Light notification sound
- **success**: Positive completion sound

### 3. Type Identification with Emojis
- **error**: ❌ or 🚨
- **warning**: ⚠️
- **info**: ℹ️ or 💡
- **success**: ✅ or 🎉