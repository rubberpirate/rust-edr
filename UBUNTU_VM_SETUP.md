# 🐧 Ubuntu VM Setup Guide - Rust EDR

## ⚠️ Current Issue
Your Ubuntu VM has Rust 1.75.0, but the project requires Rust 1.82+ due to the `indexmap` dependency.

---

## 🚀 Quick Fix: Update Rust (Recommended)

### Step 1: Update Rust to Latest Version
```bash
# Update rustup and Rust
rustup update

# Verify the new version
rustc --version
# Should show: rustc 1.83.0 (or newer)
```

### Step 2: Build the Project
```bash
cd ~/rust-edr  # or wherever you cloned it
cargo clean
cargo build --release
```

That's it! This should fix the `indexmap v2.12.0` error.

---

## 🔄 Alternative: Fresh Rust Installation

If `rustup update` doesn't work, do a fresh install:

### Step 1: Uninstall Old Rust (if needed)
```bash
rustup self uninstall
```

### Step 2: Install Latest Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Select option 1 (default installation)
# This installs the latest stable Rust
```

### Step 3: Configure Shell
```bash
source "$HOME/.cargo/env"

# Or add to ~/.bashrc or ~/.zshrc permanently:
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
source ~/.bashrc
```

### Step 4: Verify Installation
```bash
rustc --version
cargo --version
rustup --version
```

### Step 5: Build Project
```bash
cd ~/rust-edr
cargo build --release
```

---

## 🛠️ Alternative Fix: Downgrade Dependencies (Not Recommended)

If you **must** use Rust 1.75.0, you can downgrade dependencies:

```bash
cd ~/rust-edr

# Downgrade indexmap to a version compatible with Rust 1.75
cargo update indexmap --precise 2.0.0

# Try building
cargo build --release
```

**Note**: This might cause other compatibility issues. Upgrading Rust is better!

---

## 📦 Complete Ubuntu VM Setup (Fresh Start)

If you're setting up the VM from scratch:

### Step 1: Update System
```bash
sudo apt update && sudo apt upgrade -y
```

### Step 2: Install Build Dependencies
```bash
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    curl \
    jq
```

### Step 3: Install Rust (Latest)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Step 4: Clone/Copy Project
```bash
# If cloning from GitHub
git clone https://github.com/rubberpirate/rust-edr.git
cd rust-edr

# Or if copying from host machine, use shared folder
# then cd to the directory
```

### Step 5: Build
```bash
cargo build --release
```

### Step 6: Setup Directories
```bash
sudo mkdir -p /var/log/rust-edr /var/lib/rust-edr
sudo chown -R $USER:$USER /var/log/rust-edr /var/lib/rust-edr
```

### Step 7: Run!
```bash
sudo ./target/release/rust-edr start --verbose
```

---

## 🔍 Troubleshooting

### Issue: `rustup: command not found`
**Solution**: Rust not in PATH, run:
```bash
source "$HOME/.cargo/env"
```

### Issue: `rustup update` says "no update available"
**Solution**: Your rustup itself is outdated:
```bash
rustup self update
rustup update
```

### Issue: Still getting indexmap error after update
**Solution**: Clean and rebuild:
```bash
cargo clean
rm -rf ~/.cargo/registry/index/*
cargo build --release
```

### Issue: Build takes forever in VM
**Solution**: Allocate more CPU/RAM to VM in VirtualBox settings:
- Recommended: 4GB RAM, 2+ CPU cores
- Minimum: 2GB RAM, 1 CPU core

### Issue: Permission denied errors
**Solution**: 
```bash
# Fix cargo cache permissions
sudo chown -R $USER:$USER ~/.cargo

# Fix project directory
sudo chown -R $USER:$USER ~/rust-edr
```

---

## ✅ Quick Verification

After fixing Rust, verify everything works:

```bash
# 1. Check Rust version
rustc --version
# Expected: rustc 1.83.0 or newer

# 2. Check cargo
cargo --version
# Expected: cargo 1.83.0 or newer

# 3. Build project
cd ~/rust-edr
cargo build --release
# Expected: Finished in ~2-5 minutes, 0 errors

# 4. Run EDR
sudo ./target/release/rust-edr --version
# Expected: rust-edr 0.1.0

# 5. Test run
sudo ./target/release/rust-edr start
# Expected: ✅ EDR System running
```

---

## 🎯 Expected Output After Fix

```bash
$ rustc --version
rustc 1.83.0 (90b35a623 2024-11-26)

$ cargo build --release
   Compiling proc-macro2 v1.0.92
   Compiling unicode-ident v1.0.13
   Compiling libc v0.2.168
   ...
   Compiling rust-edr v0.1.0 (/home/user/rust-edr)
    Finished `release` profile [optimized] target(s) in 2m 15s

$ sudo ./target/release/rust-edr start
🛡️  Starting Rust EDR System...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Threat threshold: 7.0
Auto-response: disabled
Enabled modules: process,file,network,memory,user,rootkit
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Starting process monitor...
Starting file monitor...
Starting network monitor...
Starting memory monitor...
Starting user monitor...
Starting rootkit monitor...
✅ EDR System running. Press Ctrl+C to stop.
```

---

## 📚 Why This Happened

**Root Cause**: The `indexmap` crate version 2.12.0 requires Rust 1.82+, but Ubuntu's default Rust installation (if installed via apt) is often outdated.

**Solution**: Always install Rust via `rustup` (official Rust installer), not via `apt`, because:
- ✅ `rustup` gives you the latest Rust version
- ✅ Easy to update: just run `rustup update`
- ✅ Can switch between versions: `rustup default stable`
- ❌ `apt install rustc` gives you old, outdated versions

**Best Practice**: 
```bash
# DON'T do this:
sudo apt install rustc cargo

# DO this instead:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 🚀 TL;DR (Fastest Fix)

```bash
# In your Ubuntu VM, run these 3 commands:
rustup update
cargo clean
cargo build --release

# Done! ✅
```

If that doesn't work:
```bash
# Fresh Rust install:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
cargo build --release
```

---

**Good luck! Let me know if you hit any other issues! 🐧🦀**
