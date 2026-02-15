#!/bin/bash

cd "$(dirname "$0")/.."

echo "Stopping Torvi development environment..."

docker compose down

if tmux has-session -t torvi 2>/dev/null; then
    tmux kill-session -t torvi
    echo "Stopped tmux session"
fi

echo "All services stopped"
