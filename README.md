# VoxLAN 🎙️

**Voice of the LAN** - A powerful LAN proxy that speaks your network's language
It is a Rust-based command-line and proxy server tool that scans local TCP ports to find open services,
then starts a proxy server forwarding requests to the first detected open port.
It provides real-time feedback via terminal animations and supports forwarding HTTP requests using Actix Web and Reqwest.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

## 🚀 What is VoxLAN?

VoxLAN is a sophisticated LAN proxy server built in Rust that discovers and manages network connections. It acts as an intelligent intermediary for your local network communications, providing seamless proxy functionality with advanced network discovery capabilities.

## ✨ Features

### Current Features

- 🔍 **Network Discovery**: Automatically scans and identifies open ports on local network
- 󰐲 QR Access: Generates QR codes for quick and easy access
- 📊 **Real-time Monitoring**: Live port scanning and network status reporting
- 🔄 **Request Forwarding**: Seamless proxy functionality for local services
- 🚀 **High Performance**: Built with Actix-web for optimal performance

### 🎯 Planned Features

- 📋 **Filter Lists**: Advanced filtering capabilities for network traffic
- 🎛️ **CLI Interface**: Comprehensive command-line interface with clap integration
- 🔢 **Device Selection**: Target specific devices by number or identifier
- 📡 **Custom Request Handling**: Flexible request processing and routing
- 🔐 **Security Features**: Traffic filtering and access control

You have three options: via Cargo, via prebuilt script, or manual install.

### 🔹 1. Easiest: Install via Cargo (Recommended)

If you have Rust installed, you can install directly from [crates.io](https://crates.io):

```bash
cargo install voxlan
```

This is the most "Rusty" and portable way.  
It automatically downloads, compiles, and installs the latest version to your `$HOME/.cargo/bin`.

> If you want even faster installs with prebuilt binaries, check out [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```bash
cargo binstall voxlan
```

---

### 🔹 2. Quick Install via Script

**Alternative:** Installs the latest release binary to your system PATH.

```bash
curl -sSfL https://raw.githubusercontent.com/santoshxshrestha/voxlan/main/scripts/install.sh | bash
```

- This script will:
  1. Build `voxlan` in release mode (if Rust is present).
  2. Copy the binary to `/usr/local/bin`.
  3. Make it executable.

> **Tip:** You may need to enter your password for `sudo` privileges.

---

### 🔹 3. Manual Build & Install

If you prefer full control or want to customize the build:

1. **Clone the repository:**

   ```bash
   git clone https://github.com/santoshxshrestha/voxlan.git
   cd voxlan
   ```

2. **Build the Release Binary:**

   ```bash
   cargo build --release
   ```

   This places the binary at `target/release/voxlan`.

3. **Copy to a PATH directory (e.g., `/usr/local/bin`):**

   ```bash
   sudo cp target/release/voxlan /usr/local/bin/voxlan
   ```

4. **(Optional) Ensure executable permission:**

   ```bash
   sudo chmod +x /usr/local/bin/voxlan
   ```

5. **Run from anywhere:**

   ```bash
   voxlan
   ```

---

## 🗑️ Uninstallation

You can uninstall using the provided script or manually:

### 🔹 1. Quick Uninstall via Script

```bash
curl -sSfL https://raw.githubusercontent.com/santoshxshrestha/voxlan/main/scripts/uninstall.sh | bash
```

### 🔹 2. Manual Uninstall

Remove the binary from your PATH:

```bash
sudo rm /usr/local/bin/voxlan
```

or

```bash
sudo rm /usr/bin/voxlan
```

If you also want to remove your cloned repository:

```bash
rm -rf ~/voxlan
```

If installed with Cargo:

```bash
cargo uninstall voxlan
```

---

## 🎮 Usage

### Basic Usage

> [!NOTE]
> By default the voxlan is bound to 8081 and target would be the open port if it

```bash
# Start VoxLAN with default settings
voxlan run

# Start VoxLAN by specifying the bind-port and target-port
# is only one)
voxlan run -b 8081 -t 8080

# Start VoxLAN by targeting specific port
voxlan run -b <bind-port>

# List of all the open ports
voxlan list

# Get help for specific command
voxlan <command> -h

```

> [!NOTE]
> path should be specified without '/'
> bind-port(optional),
> default path is /

# Running client "HTTP"

```bash
# Run the client
voxlan client -b <bind-port> --path <path>

voxlan client -b <bind-port> -p <path>

# Run clint to http://localhost:8080
voxlan client -b 8080 -p echo

# Help for the client
voxlan client --help

```

# Hosting and connecting tcp connection

> [!NOTE]
> This is not available for multiple connection but a pair will work fine
> If a crertain IP is in both the allow-list and block-list it will be blocked
> If no args are passed in the allow and block lists then all the IP will be allowed

```bash
# To host a tcp connection
voxlan host -b <bind-port>


# To connect to a tcp connection
voxlan connect  -i <private-ip-of-host-machine> -t <target-port>

# To apply allow-list while hosting
voxlan host -b <bind-port> --allow-ip=IP1,IP2,IP3,...

# To apply block-list while hosting
voxlan host -b <bind-port> --block-ip=IP1,IP2,IP3,...

```

### Current Workflow

1. **Network Scan**: VoxLAN scans ports 1-10000 on localhost
2. **Discovery**: Reports all open ports found
3. **Proxy Start**: Launches HTTP proxy server on port 8081
4. **Request Forwarding**: Forwards requests to localhost:8080

## 🏗️ Architecture

```
┌────────────────┐    ┌────────────────────┐    ┌──────────────────────────┐
│    Client      │────│      VoxLAN        │────│         Target           │
│    Request     │    │      Proxy         │    │         Service          │
│                │    │ :8081(By default)  │    │ :acitve port(By default) │
└────────────────┘    └────────────────────┘    └──────────────────────────┘
```

### Core Components

- **Port Scanner**: Multi-threaded port discovery
- **HTTP Proxy**: Actix-web based proxy server
- **Request Router**: Intelligent request forwarding
- **Network Utils**: IP discovery and connection management

## 🔧 Configuration

### Default Settings

- **Proxy Port**: 8081
- **Target Port**: UserSpecified(By default opened port)
- **Scan Range**: 1-10000
- **Timeout**: 100ms per port

## 📋 Roadmap

### Phase 1: Core Enhancement ✅

- [x] Basic proxy functionality
- [x] Network discovery
- [x] HTTP request forwarding
- [x] Add option to specify port

### Phase 2: CLI Integration 🚧

- [x] Implement clap for argument parsing
- [x] Custom request configuration
- [x] Filter list implementation

### Phase 3: Advanced Features 📋

- [x] Rich terminal UI (voxlan-style)
- [ ] Real-time network monitoring
- [ ] Traffic filtering and rules
- [ ] Configuration file support
- [ ] Plugin system architecture

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Setup

```bash
# Install development dependencies
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## 📊 Performance

- **Port Scanning**: ~1000 ports in <2 seconds
- **Proxy Latency**: <5ms additional overhead
- **Memory Usage**: ~10MB baseline
- **Concurrent Connections**: 1000+ supported

## 🔍 Troubleshooting

### Common Issues

**Port Already in Use**

```bash
# Check what's using port 8081
lsof -i :8081

# Kill the process
kill -9 <PID>
```

**Permission Denied**

```bash
# Run with elevated privileges if needed
sudo voxlan
```

**Connection Timeout**

- Increase timeout in `scan_port` function
- Check firewall settings
- Verify target service is running

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Dependencies

- 🚀 Actix Web - High-performance web framework
- 🎯 Anstyle - ANSI styling for clap's help and error message formatting
- ⚙️ Clap - Command line argument parser with derive macros
- 🔳 QR2Term - Terminal QR code generator and display library
- 🌐 Reqwest - HTTP client library with JSON support
- ⚡ Tokio - Asynchronous runtime for Rust with full feature set
  ***

🐛 **Issues**: [GitHub Issues](https://github.com/santoshxshrestha/voxlan/issues)

---

<div align="center">
  <p><strong>Built with ❤️ and Rust</strong></p>
  <p>Star ⭐ this repository if you find it helpful!</p>
</div>
