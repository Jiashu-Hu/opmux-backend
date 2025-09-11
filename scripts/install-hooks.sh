#!/bin/bash
# Install git hooks for security checks

echo "🔧 Installing git hooks..."

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Create pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook to check for security issues

echo "🔒 Running pre-commit security checks..."

# Run the security check script
if [ -f "scripts/check-security.sh" ]; then
    ./scripts/check-security.sh
    if [ $? -ne 0 ]; then
        echo ""
        echo "❌ Pre-commit security check failed!"
        echo "🚨 Commit blocked to prevent security issues"
        echo ""
        echo "💡 To fix:"
        echo "   1. Remove any AUTH_DEVELOPMENT_MODE=true from files"
        echo "   2. Use environment variables for development configuration"
        echo "   3. Run: ./scripts/check-security.sh to verify fixes"
        echo ""
        exit 1
    fi
else
    echo "⚠️  Security check script not found, skipping..."
fi

echo "✅ Pre-commit security checks passed!"
EOF

# Make the hook executable
chmod +x .git/hooks/pre-commit

echo "✅ Pre-commit hook installed successfully!"
echo ""
echo "📋 The hook will now:"
echo "   • Run security checks before each commit"
echo "   • Block commits that contain development mode configurations"
echo "   • Ensure no AUTH_DEVELOPMENT_MODE=true is committed"
echo ""
echo "🔧 To bypass the hook (NOT recommended):"
echo "   git commit --no-verify"
echo ""
echo "🗑️  To remove the hook:"
echo "   rm .git/hooks/pre-commit"
