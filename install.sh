#!/bin/sh
set -e

echo "--------------------------------------------------"
echo "🛡️  Installing 'no' - The Permanent rm Shield"
echo "--------------------------------------------------"

# 1. Use the binary you just built locally
# This is faster than downloading from GitHub while you are developing
echo "🚀 Copying your new 'no' binary..."
sudo cp target/release/no /usr/local/bin/no
sudo chmod +x /usr/local/bin/no

# 2. CREATE THE SYSTEM-WIDE SYMLINK
# This forces 'rm' to point to 'no' natively
echo "🔗 Linking 'rm' to 'no'..."
sudo ln -sf /usr/local/bin/no /usr/local/bin/rm

echo "--------------------------------------------------"
echo "✅ SUCCESS!"
echo "🚀 'rm' is now natively protected by 'no'."
echo "--------------------------------------------------"

# Show the user the path immediately
which rm