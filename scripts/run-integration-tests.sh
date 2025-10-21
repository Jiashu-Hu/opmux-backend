#!/bin/bash
# Integration test runner for Executor Layer
#
# This script runs integration tests with real OpenAI API calls.
# It requires valid API credentials to be set as environment variables.
#
# Usage:
#   ./scripts/run-integration-tests.sh
#
# Environment Variables Required:
#   OPENAI_API_KEY - Valid OpenAI API key
#   OPENAI_BASE_URL - API endpoint (optional, defaults to https://api.openai.com/v1)

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Executor Layer Integration Tests${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if OPENAI_API_KEY is set
if [ -z "$OPENAI_API_KEY" ]; then
    echo -e "${RED}❌ Error: OPENAI_API_KEY is not set${NC}"
    echo ""
    echo "Please set the OPENAI_API_KEY environment variable:"
    echo "  export OPENAI_API_KEY=your-api-key-here"
    echo ""
    echo "Optionally, set OPENAI_BASE_URL for custom endpoints:"
    echo "  export OPENAI_BASE_URL=url"
    echo ""
    exit 1
fi

# Display configuration (mask API key for security)
MASKED_KEY="${OPENAI_API_KEY:0:8}...${OPENAI_API_KEY: -4}"
echo -e "${GREEN}✅ OPENAI_API_KEY is set${NC} (${MASKED_KEY})"

if [ -n "$OPENAI_BASE_URL" ]; then
    echo -e "${GREEN}✅ OPENAI_BASE_URL is set${NC} (${OPENAI_BASE_URL})"
else
    echo -e "${YELLOW}⚠️  OPENAI_BASE_URL not set${NC} (using default: https://api.openai.com/v1)"
fi

echo ""
echo -e "${BLUE}Running integration tests...${NC}"
echo ""

# Run integration tests with output
cargo test --test executor_integration_test -- --nocapture

# Check exit code
if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}✅ All integration tests passed!${NC}"
    echo -e "${GREEN}========================================${NC}"
else
    echo ""
    echo -e "${RED}========================================${NC}"
    echo -e "${RED}❌ Integration tests failed${NC}"
    echo -e "${RED}========================================${NC}"
    exit 1
fi

