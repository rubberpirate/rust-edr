#!/bin/bash
# Quick test of the EDR Dashboard in terminal mode (no TUI for testing)

echo "🧪 Testing EDR Dashboard (Console Mode)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Start server in no-TUI mode
echo "1. Starting server (console mode)..."
./target/release/edr-server --bind 127.0.0.1:8080 --no-tui &
SERVER_PID=$!
echo "   Server PID: $SERVER_PID"
sleep 2

# Start an agent
echo "2. Starting test agent..."
./target/release/edr-agent --server ws://127.0.0.1:8080 --endpoint-id test-agent-1 &
AGENT_PID=$!
echo "   Agent PID: $AGENT_PID"
sleep 3

echo ""
echo "✅ Test setup complete!"
echo ""
echo "Server and agent are running for 10 seconds..."
echo "Check if heartbeats are being sent..."
sleep 10

echo ""
echo "🛑 Stopping test..."
kill $AGENT_PID $SERVER_PID 2>/dev/null
sleep 1
echo "✅ Test complete!"
