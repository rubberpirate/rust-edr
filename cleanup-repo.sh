#!/bin/bash
# Cleanup script to remove target directories from git and reduce repo size

echo "🧹 Git Repository Cleanup Script"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Current Repository Size:"
du -sh .
echo ""
echo "📈 Breakdown:"
du -sh target edr-dashboard/target .git 2>/dev/null
echo ""

echo "🔍 Problem Detected:"
echo "   The target/ directories (Rust build artifacts) are tracked in git!"
echo "   This adds ~649 MB of unnecessary files to the repository."
echo ""
echo "   Files in git: $(git ls-files | grep 'target/' | wc -l) target files"
echo ""

read -p "❓ Do you want to remove target directories from git? (y/n): " answer

if [[ "$answer" != "y" ]]; then
    echo "❌ Cleanup cancelled"
    exit 0
fi

echo ""
echo "🚀 Starting cleanup..."
echo ""

# Step 1: Remove target directories from git index
echo "1️⃣  Removing target directories from git index..."
git rm -r --cached target/ 2>/dev/null && echo "   ✅ Removed target/" || echo "   ℹ️  target/ already removed or not present"
git rm -r --cached edr-dashboard/target/ 2>/dev/null && echo "   ✅ Removed edr-dashboard/target/" || echo "   ℹ️  edr-dashboard/target/ already removed"

echo ""
echo "2️⃣  Checking updated .gitignore..."
if grep -q '\*\*/target/' .gitignore; then
    echo "   ✅ .gitignore already has **/target/"
else
    echo "   ⚠️  .gitignore might need updating"
fi

echo ""
echo "3️⃣  Current git status:"
echo ""
git status --short

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📋 Next Steps:"
echo ""
echo "1. Review the changes:"
echo "   git status"
echo ""
echo "2. Commit the removal:"
echo "   git commit -m 'chore: remove target directories from git tracking'"
echo ""
echo "3. Clean git history to reclaim space (OPTIONAL):"
echo "   git gc --aggressive --prune=now"
echo ""
echo "4. After commit, the working directory will still have target/"
echo "   That's normal! They're just not tracked by git anymore."
echo ""
echo "5. Check new repo size:"
echo "   du -sh .git"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "✅ Cleanup script complete!"
echo ""
echo "⚠️  NOTE: The target/ directories still exist on disk (649 MB)."
echo "   They're just not tracked by git anymore."
echo "   To remove them: cargo clean (in both directories)"
echo ""
