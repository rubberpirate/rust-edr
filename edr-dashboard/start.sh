#!/bin/bash
# Quick Start Script for EDR Dashboard

echo "🛡️  EDR Dashboard Quick Start"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if binaries exist
if [[ ! -f target/release/edr-server ]] || [[ ! -f target/release/edr-agent ]]; then
    echo "❌ Binaries not found! Building project..."
    cargo build --release
    if [[ $? -ne 0 ]]; then
        echo "❌ Build failed!"
        exit 1
    fi
fi

echo "✅ Binaries ready!"
echo ""
echo "Select mode:"
echo "  1) Start Server (TUI Dashboard)"
echo "  2) Start Agent (on this machine for testing)"
echo "  3) Both (Server + Local Agent for demo)"
echo "  4) Show help"
echo ""
read -p "Choice [1-4]: " choice

case $choice in
    1)
        echo ""
        echo "🚀 Starting EDR Server..."
        echo "   TUI dashboard will appear in 2 seconds"
        echo "   Press 'q' to quit"
        echo ""
        ./target/release/edr-server --bind 0.0.0.0:8080
        ;;
    2)
        echo ""
        read -p "Server address [ws://127.0.0.1:8080]: " server
        server=${server:-ws://127.0.0.1:8080}
        read -p "Endpoint ID [test-agent]: " endpoint_id
        endpoint_id=${endpoint_id:-test-agent}
        echo ""
        echo "🚀 Starting EDR Agent..."
        echo "   Connecting to: $server"
        echo "   Endpoint ID: $endpoint_id"
        echo ""
        ./target/release/edr-agent --server "$server" --endpoint-id "$endpoint_id"
        ;;
    3)
        echo ""
        echo "🚀 Starting demo mode..."
        echo "   1. Server in background"
        echo "   2. Agent connecting to localhost"
        echo "   3. TUI dashboard will appear"
        echo ""
        echo "Starting server..."
        ./target/release/edr-server --bind 127.0.0.1:8080 &
        SERVER_PID=$!
        echo "Server PID: $SERVER_PID"
        
        sleep 2
        
        echo "Starting agent..."
        ./target/release/edr-agent --server ws://127.0.0.1:8080 --endpoint-id demo-agent &
        AGENT_PID=$!
        echo "Agent PID: $AGENT_PID"
        
        echo ""
        echo "✅ Demo running!"
        echo "   Server: http://127.0.0.1:8080"
        echo "   Press Ctrl+C to stop"
        echo ""
        
        # Wait for Ctrl+C
        trap "echo ''; echo 'Stopping...'; kill $SERVER_PID $AGENT_PID 2>/dev/null; exit" INT
        wait
        ;;
    4)
        echo ""
        echo "📚 EDR Dashboard Help"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        echo ""
        echo "SERVER (Main OS):"
        echo "  ./target/release/edr-server [OPTIONS]"
        echo ""
        echo "  Options:"
        echo "    --bind <ADDR>    Bind address (default: 0.0.0.0:8080)"
        echo "    --no-tui         Disable TUI, log to console"
        echo ""
        echo "  Example:"
        echo "    ./target/release/edr-server --bind 192.168.1.100:8080"
        echo ""
        echo "AGENT (VMs/Endpoints):"
        echo "  ./target/release/edr-agent [OPTIONS]"
        echo ""
        echo "  Options:"
        echo "    --server <URL>           Server WebSocket URL (required)"
        echo "    --endpoint-id <ID>       Unique endpoint identifier"
        echo "    --heartbeat <SECONDS>    Heartbeat interval (default: 5)"
        echo ""
        echo "  Example:"
        echo "    ./target/release/edr-agent \\"
        echo "      --server ws://192.168.1.100:8080 \\"
        echo "      --endpoint-id vm-ubuntu-01 \\"
        echo "      --heartbeat 10"
        echo ""
        echo "NETWORK SETUP:"
        echo "  1. Start server on main OS"
        echo "  2. Note server IP address (use 'ip addr' or 'ifconfig')"
        echo "  3. Copy edr-agent binary to VMs"
        echo "  4. On each VM, run agent with server's IP"
        echo "  5. Firewall: Open port 8080 on server"
        echo ""
        echo "TUI CONTROLS:"
        echo "  q - Quit dashboard"
        echo "  Auto-refreshes every 250ms"
        echo ""
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac
