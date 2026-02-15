#!/bin/bash
set -e

echo "Starting Torvi Local Development Environment"
echo ""

cd "$(dirname "$0")/.."

# Check if .env exists
if [ ! -f .env ]; then
    echo ".env file not found. Creating from .env.example..."
    cp .env.example .env
    echo ".env created. Please review and update if needed."
    echo ""
fi

# Start Docker services
echo "Starting Docker services (MongoDB + MinIO)..."
docker compose up -d

# Wait for MongoDB
echo "Waiting for MongoDB..."
until docker exec torvi-mongodb mongosh --eval "db.adminCommand('ping')" > /dev/null 2>&1; do
    sleep 2
done
echo "MongoDB ready"

# Wait for MinIO
echo "Waiting for MinIO..."
until curl -sf http://localhost:9000/minio/health/live > /dev/null 2>&1; do
    sleep 2
done
echo "MinIO ready"

# Install frontend dependencies
echo "Installing frontend dependencies..."
cd web && npm install && cd ..

echo ""
echo "All services ready!"
echo ""
echo "Services:"
echo "  MongoDB:       mongodb://localhost:27017"
echo "  MinIO API:     http://localhost:9000"
echo "  MinIO Console: http://localhost:9001 (minioadmin/minioadmin)"
echo ""
echo "Run the servers:"
echo "  Backend:  cargo run                       -> http://localhost:8000"
echo "  Landing:  cd web/landing && npm run dev   -> http://localhost:3000"
echo "  App:      cd web/app && npm run dev       -> http://localhost:5173"
echo ""
echo "Or all at once:  ./dev/run-all.sh"
