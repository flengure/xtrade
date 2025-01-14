#!/bin/bash

# Set the server details
PORT=4462
WEB_PORT=4463
SERVER_URL="http://127.0.0.1:$PORT"
CLIENT_BIN="./target/release/xtrade"

# Colors for output
GREEN="\033[0;32m"
RED="\033[0;31m"
NC="\033[0m" # No color

# Function to report results
report() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}SUCCESS:${NC} $1"
    else
        echo -e "${RED}FAIL:${NC} $1"
    fi
}

# Start the xtrade server in the background
echo "Starting xTrade server..."
#$CLIENT_BIN server --port 7762 --bind 127.0.0.1 --state state.json &
$CLIENT_BIN server --port $PORT --web-port $WEB_PORT &
SERVER_PID=$!
sleep 2 # Wait for the server to start

# Test 1: Add a bot via CLI
echo "Testing: Add a bot via CLI..."
$CLIENT_BIN offline add-bot --name "TestBot" --exchange "Binance" --trading-fee 0.1
report "CLI Add Bot"
#
## Test 2: List bots via CLI
#echo "Testing: List bots via CLI..."
#$CLIENT_BIN offline list-bots
#report "CLI List Bots"
#
## Test 3: Add a bot via API (curl)
#echo "Testing: Add a bot via API (curl)..."
#curl -sX POST "$SERVER_URL/bots" -H "Content-Type: application/json" -d '{
#    "name": "APIBot",
#    "exchange": "Coinbase",
#    "trading_fee": 0.2
#}' > /dev/null
#report "API Add Bot"
#
## Test 4: List bots via API (curl)
#echo "Testing: List bots via API (curl)..."
#curl -sX GET "$SERVER_URL/bots" | jq .
#report "API List Bots"
#
## Test 5: Update a bot via API (curl)
#echo "Testing: Update a bot via API (curl)..."
#curl -sX PUT "$SERVER_URL/bots/1" -H "Content-Type: application/json" -d '{
#    "name": "UpdatedBot",
#    "exchange": "Kraken"
#}' > /dev/null
#report "API Update Bot"
#
## Test 6: Delete a bot via CLI
#echo "Testing: Delete a bot via CLI..."
#$CLIENT_BIN offline delete-bot --bot-id "1"
#report "CLI Delete Bot"
#
## Test 7: Delete a bot via API (curl)
#echo "Testing: Delete a bot via API (curl)..."
#curl -sX DELETE "$SERVER_URL/bots/1" > /dev/null
#report "API Delete Bot"
#
## Stop the server
#echo "Stopping xTrade server..."
#kill $SERVER_PID
#wait $SERVER_PID 2>/dev/null
#
echo -e "\nAll tests completed!"
