# Security Workflow Troubleshooting Guide

This guide helps resolve common issues with the security.yml CI workflow.

## 🔧 **Common Issues and Solutions**

### 1. **Cargo Deny Configuration Error**

**Error:**
```
error[unexpected-value]: expected '["all", "workspace", "transitive", "none"]'
   ┌─ deny.toml:25:17
   │
25 │ unmaintained = "warn"
   │                 ━━━━ unexpected value
```

**Solution:**
The issue was with deprecated fields in `deny.toml`. The configuration has been updated to use the current format.

**Fixed by:**
- Removing deprecated `copyleft` and `deny` fields from `[licenses]` section
- Using proper advisory configuration format
- Adding missing licenses (`Unicode-3.0`, `Zlib`) to the allow list

### 2. **License Rejection Errors**

**Error:**
```
error[rejected]: failed to satisfy license requirements
  ┌─ registry+https://github.com/rust-lang/crates.io-index#icu_collections@2.0.0:4:12
  │
4 │ license = "Unicode-3.0"
  │            ━━━━━━━━━━━
  │            rejected: license is not explicitly allowed
```

**Solution:**
Add the missing licenses to the `allow` list in `deny.toml`:

```toml
[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "Unicode-3.0",  # Required for ICU crates
    "Zlib",         # Required for zlib-related crates
    # ... other licenses
]
```

### 3. **Multiple Crate Versions Warning**

**Warning:**
```
warning[duplicate]: found 2 duplicate entries for crate 'regex-automata'
```

**Solution:**
This is a warning, not an error. It indicates that multiple versions of the same crate are in use. This is configured to warn (not fail) in our setup:

```toml
[bans]
multiple-versions = "warn"
```

## 🛠️ **Configuration Files**

### Current Working `deny.toml`

```toml
[graph]
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc", 
    "x86_64-apple-darwin",
]
all-features = true

[advisories]
db-path = "$CARGO_HOME/advisory-dbs"
db-urls = ["https://github.com/rustsec/advisory-db"]
ignore = []

[licenses]
confidence-threshold = 0.8
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause", 
    "ISC",
    "Unicode-DFS-2016",
    "Unicode-3.0",
    "Zlib",
]

[licenses.private]
ignore = true

[bans]
multiple-versions = "warn"
wildcards = "allow"
highlight = "all"

[sources]
unknown-registry = "warn"
unknown-git = "warn"
allow-registry = ["https://github.com/rust-lang/crates.io-index"]
```

## 🧪 **Testing Locally**

### Test Individual Checks

```bash
# Test all checks
cargo deny check

# Test specific checks
cargo deny check advisories
cargo deny check licenses  
cargo deny check bans
cargo deny check sources
```

### Generate Reports

```bash
# JSON output for CI integration
cargo deny check --format json > deny-results.json

# Human-readable output
cargo deny check --format human
```

## 🔍 **Debugging Steps**

### 1. **Validate Configuration**
```bash
# Check if deny.toml is valid
cargo deny --version
cargo deny check --help
```

### 2. **Check Specific License Issues**
```bash
# List all licenses in use
cargo deny list licenses

# Check specific crate licenses
cargo tree --format "{p} {l}"
```

### 3. **Advisory Database Issues**
```bash
# Update advisory database
cargo deny fetch

# Check specific advisory
cargo deny check advisories --json
```

## 🚀 **CI/CD Integration**

### Workflow Structure

The security workflow is split into three jobs:

1. **security-audit**: Uses `cargo-audit` for vulnerability scanning
2. **dependency-check**: Uses `cargo-deny` for comprehensive dependency analysis  
3. **supply-chain-security**: Uses `cargo-vet` for supply chain verification (experimental)

### Error Handling

- Individual check steps for better error isolation
- Artifact uploads for debugging
- Continue-on-error for experimental features
- Detailed reporting with JSON outputs

### Caching Strategy

```yaml
- name: Cache cargo registry
  uses: actions/cache@v4
  with:
    path: |
      ~/.cargo/registry
      ~/.cargo/git
      target
    key: ${{ runner.os }}-cargo-deny-${{ hashFiles('**/Cargo.lock') }}
```

## 📊 **Monitoring and Alerts**

### License Compliance

- **Allowed**: Open source licenses compatible with project goals
- **Warnings**: Unused license allowances (informational only)
- **Errors**: Prohibited licenses or missing allowances

### Security Advisories

- **Daily scans**: Automated security checks
- **Immediate alerts**: On new vulnerabilities
- **JSON reports**: For integration with security tools

### Dependency Health

- **Multiple versions**: Tracked but allowed with warnings
- **Outdated packages**: Monitored in separate workflow
- **Supply chain**: Experimental verification with cargo-vet

## 🔧 **Maintenance Tasks**

### Regular Updates

1. **Update advisory database**: Happens automatically in CI
2. **Review license allowances**: Remove unused licenses quarterly
3. **Update cargo-deny**: Keep tool updated for latest features
4. **Review security reports**: Weekly review of generated artifacts

### Configuration Tuning

```bash
# Generate new configuration template
cargo deny init

# Compare with current configuration
diff deny.toml deny.toml.new
```

## 📚 **Additional Resources**

- [cargo-deny Documentation](https://embarkstudios.github.io/cargo-deny/)
- [cargo-audit Documentation](https://docs.rs/cargo-audit/)
- [RustSec Advisory Database](https://rustsec.org/)
- [SPDX License List](https://spdx.org/licenses/)

## 🆘 **Getting Help**

If you encounter issues not covered in this guide:

1. Check the [cargo-deny GitHub issues](https://github.com/EmbarkStudios/cargo-deny/issues)
2. Review the generated artifact reports in CI
3. Test the exact failing command locally
4. Check for updates to cargo-deny and related tools
