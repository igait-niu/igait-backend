#!/usr/bin/env bash
# Test script for stage 1 media conversion
# Uploads test videos and triggers processing

set -e

JOB_ID="testuser_1"
USER_ID="testuser"

echo "📹 Testing Stage 1 Media Conversion"
echo "=================================="
echo "Job ID: $JOB_ID"
echo ""

# Create test job request
cat > /tmp/stage1_test_request.json << EOF
{
  "job_id": "$JOB_ID",
  "user_id": "$USER_ID",
  "stage": "stage1_media_conversion",
  "callback_url": "http://backend:3000/api/v1/webhook/stage/1",
  "input_keys": {},
  "metadata": {}
}
EOF

echo "📄 Test request:"
cat /tmp/stage1_test_request.json
echo ""
echo ""

echo "🚀 Sending request to Stage 1..."
curl -X POST http://localhost:8081/process \
  -H "Content-Type: application/json" \
  -d @/tmp/stage1_test_request.json \
  | python3 -m json.tool || cat

echo ""
echo "✅ Request sent! Check the logs with: docker compose logs -f stage1"
