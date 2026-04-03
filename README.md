# 🚀 marvu (mvu)

**marvu** = **mar**kdown **view**er — A modern, fast markdown viewer server built with Rust and Axum.

Transform your directory into a beautiful web-based documentation browser with live-reload capabilities. Simply run `mvu` in any directory containing Markdown files, and instantly view them in your browser with a sleek, modern interface.

## ✨ Features

- **📁 Directory Browsing**: Navigate through directories with a stunning modern UI
- **📝 Markdown Rendering**: Convert `.md` and `.mkd` files to beautiful HTML on-the-fly
- **🖼️ Image Gallery**: View images with thumbnail previews and full-screen modal viewer
  - **Thumbnail Display**: Images show as 80x80px thumbnails in directory listings
  - **Modal Viewer**: Click to view full-size images in an elegant modal window
  - **Keyboard Navigation**: Use arrow keys (←/→) to browse through images
  - **Quick Access**: Direct link to original image file in the modal
  - **Format Support**: JPG, PNG, GIF, WebP, SVG (case-insensitive)
- **📦 ZIP File Support**: Browse and view contents inside ZIP archives
  - **Directory Navigation**: Explore ZIP file structure like regular directories
  - **Markdown Preview**: View markdown files inside ZIP archives
  - **Image Gallery**: View images inside ZIP files with thumbnail and modal support
  - **Seamless Integration**: ZIP contents use the same beautiful UI as regular files
- **🔄 Live Reload**: Automatic browser refresh when files change
- **🎨 Modern Design**: Sleek gradient backgrounds with glassmorphism effects
- **📱 Responsive**: Looks great on desktop and mobile devices
- **⚡ Fast**: Built with Rust for maximum performance
- **🔒 Secure**: Path traversal protection prevents unauthorized access
- **🌐 Smart Port Selection**: Automatically finds available ports
- **📑 Smart Navigation**: Markdown files feature expandable table of contents and file tree sidebar
- **🚫 Custom 404 Page**: Beautiful error page with quick navigation back home

## 🛠️ Installation

### Prerequisites

You need to install [unidoc](https://github.com/cympfh/unidoc) for markdown processing:

```bash
# Install unidoc (markdown to HTML converter)
cargo install unidoc
```

### Install marvu

```bash
# Clone and build
git clone https://github.com/cympfh/marvu
cd marvu
cargo build --release

# Or install directly from source
cargo install --path .
```

## 🚀 Usage

### Basic Usage

```bash
# Serve current directory on default port (8080)
mvu .

# Serve specific directory
mvu /path/to/your/docs

# Specify port and host
mvu --port 3000 --host 127.0.0.1 ./my-docs
```

### Command Line Options

- `--port <PORT>`: Set port number (default: 8080, auto-finds available port)
- `--host <HOST>`: Set host address (default: 0.0.0.0)
- `<DIRECTORY>`: Directory to serve (default: current directory)

### Example

```bash
$ mvu --port 8080 ./documentation
Starting server on http://0.0.0.0:8080
```

Then open your browser and navigate to `http://localhost:8080`

## 🎯 Usage Scenarios

### Viewing Images

When browsing directories, image files (JPG, PNG, GIF, WebP, SVG) are displayed as thumbnails:

1. **Click on any thumbnail** to open the full-size image in a modal viewer
2. **Navigate between images** using:
   - Left arrow key (←) or click the left navigation button
   - Right arrow key (→) or click the right navigation button
3. **Close the modal** by:
   - Clicking outside the image
   - Pressing the Escape key
   - Clicking the × button
4. **Access the original file** by clicking the "元ファイル" link in the top-right corner

### Viewing Markdown Files

Markdown files include enhanced navigation features:

1. **Table of Contents**: Click the 📑 icon in the sidebar to view document headings
2. **File Tree**: Click the 📁 icon to browse all files in the directory
3. **Smooth Scrolling**: Click any TOC item to jump to that section

### Browsing ZIP Archives

ZIP files are displayed with a 📦 icon and can be browsed like regular directories:

1. **Click on a ZIP file** to enter and view its contents
2. **Navigate directories** inside the ZIP just like regular folders
3. **View markdown files** inside ZIP archives with full formatting and styling
4. **Browse images** inside ZIP files with thumbnail display and modal viewer
5. **Use arrow keys** (←/→) to navigate between images in the ZIP archive
6. **URL format**: ZIP contents use the format `/path/to/archive.zip::internal/path`

Perfect for viewing manga, documentation archives, or any compressed content!

## 🏗️ Architecture

marvu is built with a clean, modular architecture:

- **Axum Web Framework**: High-performance async web server
- **File Watching**: Real-time monitoring using the `notify` crate
- **Server-Sent Events**: Live reload functionality
- **External Processing**: Uses `unidoc` for markdown conversion
- **Image Handling**: Native image serving with lazy loading and modal viewer
- **ZIP Archive Support**: Built-in ZIP file browsing with the `zip` crate
- **Concurrent Design**: File watching runs in separate threads
- **Modern Frontend**: Pure JavaScript with no external dependencies for the image gallery
