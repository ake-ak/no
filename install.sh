#!/bin/sh
# Exit on error
set -e

echo "--------------------------------------------------"
echo "🛡️  Installing 'no' - The Permanent rm Shield"
echo "--------------------------------------------------"

# 1. Download the latest binary
echo "📥 Downloading 'no' binary..."
sudo curl -L "https://github.com/ake-ak/no/releases/latest/download/no" -o /usr/local/bin/no
sudo chmod +x /usr/local/bin/no

# 2. CREATE THE SYSTEM-WIDE SYMLINK
# This makes 'rm' trigger 'no' automatically for EVERYONE.
# No alias required, no .profile needed!
echo "🔗 Linking 'rm' to 'no'..."
sudo ln -sf /usr/local/bin/no /usr/local/bin/rm

# 3. Handle Updates
# Since the symlink points to /usr/local/bin/no, 
# whenever the user runs this script again, it updates the binary
# and the 'rm' command stays updated automatically!

echo "--------------------------------------------------"
echo "✅ SUCCESS!"
echo "🚀 'rm' is now natively protected by 'no'."
echo "📂 Trash folder: ~/no-trash-sp"
echo "--------------------------------------------------"

# Verify immediately
echo "Current rm path: $(which rm)"