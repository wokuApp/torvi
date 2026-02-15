#!/bin/bash
set -e

cd "$(dirname "$0")/.."

if ! command -v tmux &> /dev/null; then
    echo "tmux not found. Install with: sudo apt install tmux"
    echo ""
    echo "Or run each service in a separate terminal:"
    echo "  Terminal 1: cargo run"
    echo "  Terminal 2: cd web/landing && npm run dev"
    echo "  Terminal 3: cd web/app && npm run dev"
    exit 1
fi

# Kill existing session if any
tmux kill-session -t torvi 2>/dev/null || true

tmux new-session -d -s torvi -n backend
tmux send-keys -t torvi:backend "cargo run" C-m

tmux new-window -t torvi -n landing
tmux send-keys -t torvi:landing "cd web/landing && npm run dev" C-m

tmux new-window -t torvi -n app
tmux send-keys -t torvi:app "cd web/app && npm run dev" C-m

tmux select-window -t torvi:backend

echo "All services started in tmux session 'torvi'"
echo ""
echo "  tmux attach -t torvi        -> attach to session"
echo "  Ctrl+B then 0/1/2           -> switch windows"
echo "  Ctrl+B then D               -> detach"
echo ""
echo "Services:"
echo "  [0] Backend:  http://localhost:8000"
echo "  [1] Landing:  http://localhost:3000"
echo "  [2] App:      http://localhost:5173"

tmux attach -t torvi
