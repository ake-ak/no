#!/bin/sh
set -e

# 1. Download and Install Binary
echo "🛡️ Installing 'no' to /usr/local/bin..."
sudo curl -L "https://github.com/ake-ak/no/releases/latest/download/no" -o /usr/local/bin/no
sudo chmod +x /usr/local/bin/no

# 2. Define the Alias block
# We use a unique comment so we don't add it multiple times
ALIAS_BLOCK=$(cat << 'EOF'

# --- 'no' safe-rm alias ---
alias rm='/usr/local/bin/no'
# --------------------------
EOF
)

# 3. Identify shell config files
# .bashrc (Bash), .zshrc (Zsh), .profile (Alpine/Sh)
CONFIG_FILES="$HOME/.bashrc $HOME/.zshrc $HOME/.profile"

echo "reconfiguring shell aliases..."

for FILE in $CONFIG_FILES; do
    if [ -f "$FILE" ]; then
        # Check if the alias is already there to avoid duplicates
        if ! grep -q "alias rm='/usr/local/bin/no'" "$FILE"; then
            echo "$ALIAS_BLOCK" >> "$FILE"
            echo "✅ Added alias to $FILE"
        else
            echo "ℹ️ Alias already exists in $FILE, skipping."
        fi
    fi
done



echo ""
echo "🎉 Installation Successful!"
echo "⚠️  To activate NOW, run: source ~/.profile (or restart your terminal)"

echo "--------------------------------------------------"
echo "✅ 'no' is now installed and the alias is set!"
echo "🚀 To start using it RIGHT NOW, run this command:"
echo ""
echo "   source $HOME/.profile"
echo ""
echo "Or just close this terminal and open a new one."
echo "--------------------------------------------------"