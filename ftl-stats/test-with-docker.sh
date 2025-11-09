#!/bin/bash
# Script helper per testare ftl-stats con Pi-hole in container

set -e

CONTAINER_NAME="${1:-pihole}"
EXAMPLE="${2:-summary}"

echo "🔍 Looking for Pi-hole container: $CONTAINER_NAME"

# Check if container exists and is running
if ! docker ps --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
    echo "❌ Container '$CONTAINER_NAME' not found or not running"
    echo ""
    echo "Usage: $0 [CONTAINER_NAME] [EXAMPLE]"
    echo "  CONTAINER_NAME: Docker container name (default: pihole)"
    echo "  EXAMPLE: Example to run (default: summary)"
    echo ""
    echo "Examples:"
    echo "  $0 pihole summary"
    echo "  $0 pihole basic"
    exit 1
fi

# Get FTL PID from container
echo "📋 Getting FTL PID from container..."
FTL_PID=$(docker exec "$CONTAINER_NAME" pidof pihole-FTL 2>/dev/null || docker exec "$CONTAINER_NAME" cat /run/pihole-FTL.pid 2>/dev/null)

if [ -z "$FTL_PID" ]; then
    echo "❌ Could not find FTL PID in container"
    exit 1
fi

echo "✓ Found FTL PID: $FTL_PID"

# Check if shared memory files exist on host
echo "🔍 Checking shared memory files..."
if ls /dev/shm/FTL-$FTL_PID-* >/dev/null 2>&1; then
    echo "✓ Shared memory files found on host:"
    ls -lh /dev/shm/FTL-$FTL_PID-* | head -5
    echo "  ... (showing first 5 files)"

    # Run the example
    echo ""
    echo "🚀 Running example: $EXAMPLE"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    cargo run --example "$EXAMPLE" "$FTL_PID"
else
    echo "❌ Shared memory files NOT found on host"
    echo ""
    echo "Make sure /dev/shm is mounted from host to container:"
    echo ""
    echo "  docker-compose.yml:"
    echo "    services:"
    echo "      pihole:"
    echo "        volumes:"
    echo "          - /dev/shm:/dev/shm"
    echo ""
    echo "Or check if container is using a different tmpfs:"
    docker exec "$CONTAINER_NAME" ls -la /dev/shm/FTL-* 2>/dev/null || echo "  No FTL files in container /dev/shm either"
    exit 1
fi
