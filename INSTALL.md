# Installation Guide

## Installing Rust

If you don't have Rust installed, run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:
```bash
rustc --version
cargo --version
```

## System Dependencies

### Ubuntu/Debian
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev

# For eBPF support (optional)
sudo apt install -y clang llvm libbpf-dev linux-headers-$(uname -r)
```

### Fedora/RHEL/CentOS
```bash
sudo dnf install -y gcc pkg-config openssl-devel

# For eBPF support (optional)
sudo dnf install -y clang llvm libbpf-devel kernel-devel
```

### Arch Linux
```bash
sudo pacman -S base-devel openssl pkg-config

# For eBPF support (optional)
sudo pacman -S clang llvm libbpf linux-headers
```

## Building the Project

```bash
# Build in debug mode (faster compilation)
cargo build

# Build in release mode (optimized)
cargo build --release

# Build without eBPF support
cargo build --release --no-default-features

# Run without installing
cargo run -- --help

# Install system-wide
cargo install --path .
```

## Troubleshooting

### Missing kernel headers
```bash
# Ubuntu/Debian
sudo apt install linux-headers-$(uname -r)

# Fedora
sudo dnf install kernel-devel-$(uname -r)
```

### Permission denied errors
The EDR requires root privileges:
```bash
sudo ./target/release/rust-edr start
```

### Build errors
Update Rust to the latest version:
```bash
rustup update stable
```
