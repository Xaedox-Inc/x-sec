# X-Sec - Modular Security Penetration Testing Tool

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey.svg)

X-Sec is a modern, modular security penetration testing tool built with Rust and Tauri. It provides a flexible framework for implementing security scanning modules with both CLI and GUI interfaces.

## 🌟 Features

- **Modular Architecture**: Easy-to-extend plugin system for security modules
- **Cross-Platform**: Works on Linux, macOS, and Windows
- **Dual Interface**: Both command-line and graphical user interface
- **Async Scanning**: Efficient concurrent scanning with Tokio
- **Scan History**: Built-in database for tracking scan results
- **Type-Safe**: Built with Rust for memory safety and performance

## 🏗️ Architecture

X-Sec follows a workspace-based architecture:

```
x-sec/
├── x-sec-core/          # Core library with traits and types
├── x-sec-modules/       # Security module implementations
├── x-sec-cli/           # Command-line interface
├── src-tauri/           # Tauri backend for GUI
└── src/                 # Frontend TypeScript code
```

### Components

- **x-sec-core**: Defines core traits (`SecurityModule`) and types (`Target`, `ScanResult`, `Finding`, etc.)
- **x-sec-modules**: Implements security scanning modules (Port Scanner, etc.)
- **x-sec-cli**: Command-line interface for running scans
- **src-tauri**: Tauri backend exposing Rust functions to the frontend
- **Frontend**: Vanilla TypeScript UI built with Vite

## 📋 Prerequisites

### System Requirements

- **Rust**: 1.70 or higher
- **Node.js**: 16.x or higher
- **npm**: 8.x or higher

### Platform-Specific Dependencies

**Linux (Ubuntu/Debian)**:
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev \
  libayatana-appindicator3-dev librsvg2-dev libssl-dev
```

**macOS**:
```bash
# Xcode Command Line Tools
xcode-select --install
```

**Windows**:
- Microsoft C++ Build Tools
- WebView2 (usually pre-installed on Windows 10/11)

## 🚀 Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/Xaedox-Inc/x-sec.git
cd x-sec
```

### 2. Build the Project

```bash
# Build all Rust workspace members
cargo build --release

# Install frontend dependencies
npm install
```

### 3. Run the CLI

```bash
# List available modules
./target/release/x-sec list-modules

# Run a port scan
./target/release/x-sec scan --module port_scanner --target 192.168.1.1

# Scan a specific port
./target/release/x-sec scan --module port_scanner --target example.com --port 443
```

### 4. Run the GUI Application

```bash
# Development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## 📦 Available Modules

### Port Scanner
Scans for open TCP ports on target hosts.

**Features**:
- Scans common ports: 21 (FTP), 22 (SSH), 23 (Telnet), 25 (SMTP), 80 (HTTP), 443 (HTTPS), 3306 (MySQL), 5432 (PostgreSQL), 8080 (HTTP-Alt)
- Configurable timeout and concurrency
- Async concurrent scanning for performance

**Usage**:
```bash
x-sec scan --module port_scanner --target 192.168.1.1
```

## 🔧 Development

### Project Structure

```
x-sec/
├── Cargo.toml                 # Workspace root
├── README.md
├── .gitignore
├── x-sec-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs            # Core traits and types
│       └── db.rs             # Database functions
├── x-sec-modules/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── port_scanner.rs   # Port scanner implementation
├── x-sec-cli/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs           # CLI implementation
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json       # Tauri configuration
│   ├── build.rs
│   └── src/
│       └── main.rs           # Tauri commands
├── src/
│   ├── main.ts               # Frontend TypeScript
│   └── style.css             # Frontend styling
├── index.html
├── package.json
├── tsconfig.json
└── vite.config.ts
```

### Adding a New Module

1. Create a new module file in `x-sec-modules/src/`:

```rust
use x_sec_core::{SecurityModule, Target, ScanResult, ModuleError, ModuleConfig};

#[derive(Debug, Clone)]
pub struct MyModule {
    config: ModuleConfig,
}

impl MyModule {
    pub fn new() -> Self {
        Self {
            config: ModuleConfig::default(),
        }
    }
}

#[async_trait::async_trait]
impl SecurityModule for MyModule {
    fn name(&self) -> &str {
        "my_module"
    }
    
    fn description(&self) -> &str {
        "Description of what the module does"
    }
    
    fn version(&self) -> &str {
        "0.1.0"
    }
    
    fn configure(&mut self, config: ModuleConfig) -> Result<(), ModuleError> {
        self.config = config;
        Ok(())
    }
    
    async fn scan(&self, target: &Target) -> Result<ScanResult, ModuleError> {
        // Implement scanning logic
        todo!()
    }
    
    fn validate_target(&self, target: &Target) -> Result<(), ModuleError> {
        // Validate target
        Ok(())
    }
}
```

2. Export the module in `x-sec-modules/src/lib.rs`:

```rust
pub mod my_module;
pub use my_module::MyModule;
```

3. Add the module to the CLI and Tauri backend

### Building for Production

```bash
# Build optimized Rust binaries
cargo build --release

# Build Tauri application for all platforms
npm run tauri build
```

## 🧪 Testing

```bash
# Run all tests
cargo test --workspace

# Run specific module tests
cargo test -p x-sec-modules

# Run CLI tests
cargo test -p x-sec-cli
```

## 📝 Configuration

### Module Configuration

Modules can be configured with custom settings:

```rust
let mut scanner = PortScanner::new();
let config = ModuleConfig {
    timeout_ms: 3000,
    concurrent_scans: 20,
    retry_count: 2,
    custom: HashMap::new(),
};
scanner.configure(config)?;
```

### Database

Scan results are automatically stored in a SQLite database (`x-sec.db`). The database tracks:
- Scan history
- Findings per scan
- Module execution metadata

## 🛣️ Roadmap

- [ ] Additional security modules:
  - [ ] HTTP header analysis
  - [ ] SSL/TLS certificate validation
  - [ ] Directory enumeration
  - [ ] Vulnerability scanning
- [ ] Export results (JSON, CSV, PDF)
- [ ] Scheduled scanning
- [ ] Network topology mapping
- [ ] Integration with vulnerability databases (CVE, CVSS)
- [ ] Custom module templates
- [ ] Plugin marketplace

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Guidelines

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow Rust formatting guidelines (use `cargo fmt`)
- Run clippy before submitting (`cargo clippy`)
- Add tests for new features
- Update documentation as needed

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This tool is intended for authorized security testing and educational purposes only. Users are responsible for obtaining proper authorization before scanning any systems. Unauthorized scanning may be illegal in your jurisdiction.

## 🔐 Security

If you discover a security vulnerability, please email security@xaedox.com. Do not open a public issue.

## 📞 Support

- Issues: [GitHub Issues](https://github.com/Xaedox-Inc/x-sec/issues)
- Discussions: [GitHub Discussions](https://github.com/Xaedox-Inc/x-sec/discussions)

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- UI powered by [Tauri](https://tauri.app/)
- Frontend built with [Vite](https://vitejs.dev/)
- Async runtime: [Tokio](https://tokio.rs/)

---

Made with ❤️ by Xaedox Inc.