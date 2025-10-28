# 🧹 Repository Size Analysis

## Problem Identified

Your repository is **776 MB** which is quite large for a Rust project. Here's the breakdown:

### Size Breakdown:
```
Total Repo:                 776 MB
├── target/                 339 MB  ⚠️  TRACKED IN GIT!
├── edr-dashboard/target/   310 MB  ⚠️  TRACKED IN GIT!
├── .git/                   127 MB
└── source code             <1 MB
```

### Root Cause:

**1,404 files** from `target/` directories (Rust build artifacts) were accidentally committed to git!

## Why This Happened

The original `.gitignore` had:
```gitignore
/target/     # Only ignores /target/ at repo root
```

But this **doesn't** ignore:
- `edr-dashboard/target/`
- Any other nested target directories

## Solution

### ✅ Fixed `.gitignore`

Changed from:
```gitignore
/target/
Cargo.lock
```

To:
```gitignore
# Rust build artifacts (all target directories)
target/
**/target/
Cargo.lock
```

Now **all** `target/` directories are ignored, including nested ones.

### 🚀 Cleanup Steps

#### Quick Cleanup (Run the script):
```bash
./cleanup-repo.sh
```

#### Manual Cleanup:

**1. Remove target directories from git:**
```bash
# Remove from git index (keeps files on disk)
git rm -r --cached target/
git rm -r --cached edr-dashboard/target/
```

**2. Check what will be removed:**
```bash
git status
```

**3. Commit the changes:**
```bash
git commit -m "chore: remove target directories from git tracking"
```

**4. (Optional) Clean git history to reclaim space:**
```bash
git gc --aggressive --prune=now
```

**5. Check new .git size:**
```bash
du -sh .git
```

### Expected Results:

After cleanup:
- `.git/` directory will shrink significantly (from 127 MB to ~50 MB or less)
- `target/` directories still exist on disk but are **no longer tracked**
- Future commits won't include build artifacts
- Repository will be much smaller when cloned

## Additional Cleanup (Optional)

If you want to free up disk space, you can also delete the build artifacts:

```bash
# Clean main EDR build artifacts
cd /home/rubberpirate/rust-edr
cargo clean        # Removes target/ (339 MB freed)

# Clean dashboard build artifacts
cd /home/rubberpirate/rust-edr/edr-dashboard
cargo clean        # Removes target/ (310 MB freed)
```

**Note**: You'll need to rebuild next time:
```bash
cargo build --release
```

## Summary

| Item | Before | After Cleanup | After cargo clean |
|------|--------|---------------|-------------------|
| Total Size | 776 MB | ~127 MB | ~1 MB |
| .git/ | 127 MB | ~50 MB | ~50 MB |
| target/ | 339 MB | 339 MB* | 0 MB |
| edr-dashboard/target/ | 310 MB | 310 MB* | 0 MB |

\* = Still on disk, but not tracked by git

## Prevention

The updated `.gitignore` now has:
- ✅ `target/` - Ignores target at any level
- ✅ `**/target/` - Explicitly ignores nested target directories
- ✅ `Cargo.lock` - Standard Rust practice for binaries

This prevents build artifacts from ever being committed again.

## Quick Reference

```bash
# Run cleanup script (recommended)
./cleanup-repo.sh

# Check repo size
du -sh .
du -sh .git

# See what's tracked in git
git ls-files | grep target | wc -l

# Remove build artifacts from disk
cargo clean
cd edr-dashboard && cargo clean

# Rebuild when needed
cargo build --release
```

## Files Modified

- ✅ `.gitignore` - Updated to ignore all target directories
- ✅ `cleanup-repo.sh` - Interactive cleanup script created

Run `./cleanup-repo.sh` to fix the issue now!
