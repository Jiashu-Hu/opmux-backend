# Security Guidelines

## 🔒 Authentication Security

This project implements strict security measures to prevent accidental deployment of development
configurations in production.

## ⚠️ Critical Security Rules

### **NEVER commit code with development mode enabled**

❌ **FORBIDDEN:**

```bash
# These will be blocked by CI
AUTH_DEVELOPMENT_MODE=true
development_mode = true
```

✅ **ALLOWED:**

```bash
# Use environment variables for development
export AUTH_DEVELOPMENT_MODE=true  # Local only
cargo run -p gateway
```

## 🛡️ Security Checks

### Automated CI Checks

Every push and pull request runs security checks that will **BLOCK deployment** if:

1. **Development mode is enabled** in any committed file
2. **Hardcoded development settings** are found in Rust code
3. **`.env` files** are accidentally committed
4. **Environment variables** contain development settings in CI

### Local Security Checks

Run security checks locally:

```bash
# Check for security issues
./scripts/check-security.sh

# Install pre-commit hooks (recommended)
./scripts/install-hooks.sh
```

## 🔧 Safe Development Practices

### Method 1: Environment Variables (Recommended)

```bash
# Development
AUTH_DEVELOPMENT_MODE=true cargo run -p gateway

# Production (default)
cargo run -p gateway
```

### Method 2: Local .env File

```bash
# Create local .env (already in .gitignore)
echo "AUTH_DEVELOPMENT_MODE=true" > .env
echo "AUTH_DEV_CLIENT_ID=my-dev-client" >> .env

# Run normally
cargo run -p gateway
```

### Method 3: IDE Configuration

Configure your IDE to set environment variables for development.

## 🚨 What Happens When Security Checks Fail

### CI Pipeline

- ❌ **Build fails immediately**
- ❌ **Deployment is blocked**
- ❌ **Pull request cannot be merged**

### Pre-commit Hook

- ❌ **Commit is rejected**
- ❌ **Must fix issues before committing**

### Error Messages

```
❌ SECURITY VIOLATION: Found AUTH_DEVELOPMENT_MODE=true in committed files
   This could disable authentication in production!
```

## 🔧 How to Fix Security Violations

1. **Remove development settings** from committed files
2. **Use environment variables** instead
3. **Verify with security check:**
   ```bash
   ./scripts/check-security.sh
   ```

## 📋 Security Checklist

Before committing:

- [ ] No `AUTH_DEVELOPMENT_MODE=true` in any committed files
- [ ] No hardcoded `development_mode = true` in Rust code
- [ ] `.env` files are not committed (check `.gitignore`)
- [ ] Run `./scripts/check-security.sh` locally

## 🏗️ Production Deployment

Production deployments automatically:

- ✅ **Enforce production mode** (`AUTH_DEVELOPMENT_MODE=false`)
- ✅ **Verify security configuration**
- ✅ **Block if development mode detected**

## 🆘 Emergency Override

**⚠️ ONLY for emergencies and with approval:**

```bash
# Bypass pre-commit hook (NOT recommended)
git commit --no-verify

# Disable CI security check (requires admin approval)
# Add [skip security] to commit message
```

## 📞 Support

If you encounter security check issues:

1. Review this document
2. Run `./scripts/check-security.sh` for details
3. Contact the security team if needed

## 🔄 Regular Security Audits

- **Weekly:** Automated security scans
- **Monthly:** Manual security review
- **Release:** Full security audit

Remember: **Security is everyone's responsibility!** 🔒
