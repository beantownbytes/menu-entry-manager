# Menu Entry Manager

A modern GUI application for creating and managing desktop files (`.desktop` files) on Linux systems. Built with Rust, GTK4, and libadwaita for a native Linux desktop experience.

## Features

- **Create New Desktop Files**: Easily create new desktop entries for applications, links, or directories
- **Edit Existing Files**: Load and edit existing desktop files from system and user directories
- **Visual Editor**: Intuitive form-based interface for all desktop file properties
- **Validation**: Built-in validation ensures your desktop files meet the freedesktop.org specification
- **Multiple Types**: Support for Application, Link, and Directory desktop entry types
- **Modern UI**: Clean, modern interface using GTK4 and libadwaita

## What are Desktop Files?

Desktop files (`.desktop` files) are configuration files that define how applications appear in Linux desktop environments. They specify:

- Application name and description
- Command to execute
- Icon to display
- Categories for menu organization
- Whether to run in terminal
- And many other properties

These files are typically stored in:
- `/usr/share/applications/` (system-wide)
- `~/.local/share/applications/` (user-specific)

## Installation

### Prerequisites

You'll need to install the GTK4 development libraries. On Fedora:

```bash
sudo dnf install gtk4-devel libadwaita-devel
```

On Ubuntu/Debian:

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
```

On Arch Linux:

```bash
sudo pacman -S gtk4 libadwaita
```

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd menu-entry-manager
```

2. Build the application:
```bash
cargo build --release
```

3. Run the application:
```bash
./target/release/menu-entry-manager
```

## Usage

### Creating a New Desktop File

1. Click the "New" button (document icon) in the left panel
2. Fill in the required fields:
   - **Name**: The display name of your application
   - **Type**: Choose Application, Link, or Directory
   - **Exec**: The command to execute (for Application type)
   - **URL**: The URL to open (for Link type)
3. Optionally fill in additional fields like description, icon, categories, etc.
4. Click "Save" to create the desktop file

### Editing an Existing Desktop File

1. Select a desktop file from the list in the left panel
2. The form will be populated with the current values
3. Make your changes
4. Click "Save" to update the file

### Desktop File Properties

#### Basic Information
- **Name**: Required. The display name of the application
- **Type**: Required. Application, Link, or Directory
- **Exec**: Required for Application type. The command to execute
- **Comment**: Optional description

#### Application Settings
- **Icon**: Path to icon file or icon name
- **Working Directory**: Directory to run the application from
- **Run in Terminal**: Whether to run the application in a terminal
- **MIME Types**: File types this application can handle

#### Link Settings
- **URL**: Required for Link type. The URL to open

#### Categories & Keywords
- **Categories**: Semicolon-separated list of categories (e.g., "Utility;System;")
- **Keywords**: Semicolon-separated list of keywords for search

#### Visibility
- **Hidden**: Whether to hide the entry from menus

## Desktop File Specification

This application follows the [freedesktop.org Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html).

### Common Categories

- `AudioVideo` - Multimedia applications
- `Development` - Programming tools
- `Education` - Educational software
- `Game` - Games
- `Graphics` - Image editing and viewing
- `Network` - Internet applications
- `Office` - Productivity software
- `Settings` - System configuration
- `System` - System utilities
- `Utility` - Accessories and utilities

### Example Desktop File

```ini
[Desktop Entry]
Type=Application
Name=My Custom App
Comment=A custom application
Exec=/usr/bin/my-app
Icon=my-app-icon
Terminal=false
Categories=Utility;
```

## Development

### Project Structure

- `src/main.rs` - Application entry point
- `src/app.rs` - Main application logic
- `src/desktop_file.rs` - Desktop file parsing and manipulation
- `src/ui.rs` - GTK4 user interface components

### Dependencies

- `gtk4` - GTK4 bindings for Rust
- `libadwaita` - Modern GTK4 widgets
- `serde` - Serialization/deserialization
- `anyhow` - Error handling
- `thiserror` - Custom error types

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [GTK4](https://www.gtk.org/) and [libadwaita](https://gitlab.gnome.org/GNOME/libadwaita)
- Follows the [freedesktop.org Desktop Entry Specification](https://specifications.freedesktop.org/desktop-entry-spec/)
- Inspired by the need for better desktop file management tools on Linux 