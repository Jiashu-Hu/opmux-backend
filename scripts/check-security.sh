#!/bin/bash
# Security check script for CI/CD pipeline
# Ensures no development mode configurations are accidentally committed

set -e

echo "🔒 Running security checks..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track if any security issues are found
SECURITY_ISSUES=0

echo "📋 Checking for development mode configurations..."

# Check 1: Look for AUTH_DEVELOPMENT_MODE=true in any files (excluding safe files)
echo "  🔍 Checking for AUTH_DEVELOPMENT_MODE=true..."
if grep -r "AUTH_DEVELOPMENT_MODE=true" . \
    --exclude-dir=.git \
    --exclude-dir=target \
    --exclude-dir=scripts \
    --exclude="*.md" \
    --exclude="*.example" \
    --exclude="*.yml" \
    --exclude="*.yaml" \
    --exclude="check-security.sh" \
    --exclude="install-hooks.sh" 2>/dev/null; then
    echo -e "${RED}❌ SECURITY VIOLATION: Found AUTH_DEVELOPMENT_MODE=true in committed files${NC}"
    echo -e "${RED}   This could disable authentication in production!${NC}"
    SECURITY_ISSUES=$((SECURITY_ISSUES + 1))
else
    echo -e "${GREEN}  ✅ No AUTH_DEVELOPMENT_MODE=true found in source files${NC}"
fi

# Check 2: Look for hardcoded development mode in Rust code
echo "  🔍 Checking for hardcoded development mode in Rust code..."
if grep -r "development_mode.*=.*true" --include="*.rs" . 2>/dev/null; then
    echo -e "${RED}❌ SECURITY VIOLATION: Found hardcoded development_mode=true in Rust code${NC}"
    echo -e "${RED}   This could force development mode to be always enabled!${NC}"
    SECURITY_ISSUES=$((SECURITY_ISSUES + 1))
else
    echo -e "${GREEN}  ✅ No hardcoded development mode found${NC}"
fi

# Check 3: Look for .env files that might contain development settings
echo "  🔍 Checking for committed .env files..."
if find . -name ".env" -not -path "./.git/*" -not -path "./target/*" | grep -q .; then
    echo -e "${YELLOW}⚠️  WARNING: Found .env files in repository${NC}"
    find . -name ".env" -not -path "./.git/*" -not -path "./target/*" | while read -r file; do
        echo -e "${YELLOW}    Found: $file${NC}"
        if grep -q "AUTH_DEVELOPMENT_MODE=true" "$file" 2>/dev/null; then
            echo -e "${RED}❌ SECURITY VIOLATION: $file contains AUTH_DEVELOPMENT_MODE=true${NC}"
            SECURITY_ISSUES=$((SECURITY_ISSUES + 1))
        fi
    done
else
    echo -e "${GREEN}  ✅ No .env files found in repository${NC}"
fi

# Check 4: Verify .env is in .gitignore
echo "  🔍 Checking .gitignore configuration..."
if [ -f .gitignore ]; then
    if grep -q "^\.env$" .gitignore || grep -q "^\.env" .gitignore; then
        echo -e "${GREEN}  ✅ .env is properly ignored in .gitignore${NC}"
    else
        echo -e "${YELLOW}⚠️  WARNING: .env should be added to .gitignore${NC}"
        echo -e "${YELLOW}   Add '.env' to .gitignore to prevent accidental commits${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  WARNING: No .gitignore file found${NC}"
fi

# Check 5: Look for test files that might have development mode enabled
echo "  🔍 Checking test files for development mode..."
if grep -r "AUTH_DEVELOPMENT_MODE.*true" --include="*test*" --include="*spec*" . 2>/dev/null; then
    echo -e "${YELLOW}⚠️  WARNING: Found AUTH_DEVELOPMENT_MODE=true in test files${NC}"
    echo -e "${YELLOW}   Ensure test files don't affect production configuration${NC}"
else
    echo -e "${GREEN}  ✅ No development mode found in test files${NC}"
fi

# Check 6: Verify default configuration is secure
echo "  🔍 Checking default configuration security..."
if grep -r "development_mode.*=.*false" --include="*.rs" . 2>/dev/null | grep -q "default"; then
    echo -e "${GREEN}  ✅ Default configuration appears secure${NC}"
else
    echo -e "${YELLOW}⚠️  WARNING: Could not verify default configuration is secure${NC}"
fi

echo ""
echo "📊 Security check summary:"
if [ $SECURITY_ISSUES -eq 0 ]; then
    echo -e "${GREEN}✅ All security checks passed!${NC}"
    echo -e "${GREEN}🔒 No development mode configurations found in committed code${NC}"
    exit 0
else
    echo -e "${RED}❌ Found $SECURITY_ISSUES security issue(s)${NC}"
    echo -e "${RED}🚨 BLOCKING DEPLOYMENT: Fix security issues before proceeding${NC}"
    echo ""
    echo -e "${YELLOW}💡 How to fix:${NC}"
    echo -e "${YELLOW}   1. Remove any AUTH_DEVELOPMENT_MODE=true from committed files${NC}"
    echo -e "${YELLOW}   2. Remove any hardcoded development_mode=true from Rust code${NC}"
    echo -e "${YELLOW}   3. Add .env to .gitignore if not already present${NC}"
    echo -e "${YELLOW}   4. Use environment variables for development configuration${NC}"
    exit 1
fi
