#!/bin/bash
# Debug WebSocket Connection Issues

echo "🔍 WebSocket Connection Debugger"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if binaries exist
if [[ ! -f target/release/edr-server ]] || [[ ! -f target/release/edr-agent ]]; then
    echo "❌ Binaries not found! Run: cargo build --release"
    exit 1
fi

echo "1️⃣  Starting server on 127.0.0.1:8080 (no-tui mode)..."
./target/release/edr-server --no-tui --bind 127.0.0.1:8080 &
SERVER_PID=$!
echo "   Server PID: $SERVER_PID"
sleep 2

echo ""
echo "2️⃣  Checking if server is listening..."
if netstat -tuln 2>/dev/null | grep -q ':8080.*LISTEN'; then
    echo "   ✅ Server is listening on port 8080"
elif ss -tuln 2>/dev/null | grep -q ':8080.*LISTEN'; then
    echo "   ✅ Server is listening on port 8080"
else
    echo "   ⚠️  Cannot verify if server is listening"
fi

echo ""
echo "3️⃣  Testing WebSocket connection..."
sleep 1

# Try to connect agent
echo "   Starting agent..."
timeout 5 ./target/release/edr-agent --server ws://127.0.0.1:8080 --endpoint-id debug-test 2>&1 &
AGENT_PID=$!

# Wait a bit
sleep 3

echo ""
echo "4️⃣  Checking connection status..."
if ps -p $AGENT_PID > /dev/null 2>&1; then
    echo "   ✅ Agent is still running (connection likely successful)"
    kill $AGENT_PID 2>/dev/null
else
    echo "   ❌ Agent stopped (connection failed)"
fi

echo ""
echo "5️⃣  Stopping server..."
kill $SERVER_PID 2>/dev/null
wait $SERVER_PID 2>/dev/null

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📋 Common Issues:"
echo ""
echo "1. If connection fails on 127.0.0.1:"
echo "   • Make sure no other service is using port 8080"
echo "   • Try different port: --bind 127.0.0.1:9090"
echo ""
echo "2. If connecting from another machine/VM:"
echo "   • Use server's actual IP (not 127.0.0.1)"
echo "   • Server: --bind 0.0.0.0:8080"
echo "   • Agent: --server ws://ACTUAL_IP:8080"
echo "   • Check firewall: sudo ufw allow 8080/tcp"
echo ""
echo "3. WebSocket handshake errors:"
echo "   • Make sure versions match (rebuild both)"
echo "   • Check for proxy/firewall blocking WebSocket"
echo ""
echo "4. From your screenshot (IP 10.1.33.187):"
echo "   • You're connecting from a different network"
echo "   • Server must bind to 0.0.0.0 or specific IP"
echo "   • Agent needs: ws://SERVER_IP:8080 (not ws://localhost)"
echo ""
