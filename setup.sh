#!/bin/bash
# Setup script for Rust EDR
# Creates all necessary directories and fixes permissions

echo "🛡️  Rust EDR - Setup Script"
echo "================================"
echo ""

# Create log directories
echo "📁 Creating log directories..."
sudo mkdir -p /var/log/rust-edr/archives/{threats,snapshots,sessions}
echo "✓ Created /var/log/rust-edr/archives/"

# Create data directory
echo "📁 Creating data directory..."
sudo mkdir -p /var/lib/rust-edr
echo "✓ Created /var/lib/rust-edr/"

# Fix permissions
echo "🔧 Setting permissions..."
sudo chown -R $USER:$USER /var/log/rust-edr
sudo chown -R $USER:$USER /var/lib/rust-edr
echo "✓ Permissions set for user: $USER"

echo ""
echo "✅ Setup complete!"
echo ""
echo "Directory structure:"
echo "/var/log/rust-edr/"
echo "├── archives/"
echo "│   ├── threats/      (compressed threat archives)"
echo "│   ├── snapshots/    (forensic snapshots)"
echo "│   └── sessions/     (investigation sessions)"
echo "└── *.jsonl           (active log files)"
echo ""
echo "/var/lib/rust-edr/"
echo "└── events.db         (persistent database)"
echo ""
echo "🚀 Ready to start EDR!"
echo ""
echo "Next steps:"
echo "  1. Build: cargo build --release"
echo "  2. Start: sudo ./target/release/rust-edr start --verbose"
echo "     OR"
echo "     Start dashboard: ./start_dashboard_tmux.sh"
echo ""
