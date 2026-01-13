#!/bin/sh
set -e

echo "🛡️ Installing 'no' - The Safe-rm Tool..."

# 1. Install the Binary
sudo curl -L "https://github.com/ake-ak/no/releases/latest/download/no" -o /usr/local/bin/no
sudo chmod +x /usr/local/bin/no

# 2. Identify the Alias
ALIAS_LINE="alias rm='/usr/local/bin/no'"

# 3. Apply to all possible config files
# This covers Bash, Zsh, and Sh
for FILE in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
    if [ -f "$FILE" ]; then
        if ! grep -q "alias rm=" "$FILE"; then
            echo "" >> "$FILE"
            echo "# no safe-rm alias" >> "$FILE"
            echo "$ALIAS_LINE" >> "$FILE"
            echo "✅ Added alias to $FILE"
        fi
    fi
done

# 4. Special Fix for Alpine/Sh/Ash
# This ensures the alias actually loads in new windows
if [ -f "$HOME/.profile" ]; then
    if ! grep -q "export ENV=" "$HOME/.profile"; then
        echo "export ENV=\$HOME/.profile" >> "$HOME/.profile"
        echo "✅ Configured ENV for sh/ash"
    fi
fi

echo "--------------------------------------------------"
echo "🎉 Done! Installation finished."
echo "🚀 To activate NOW, run: . ~/.profile"
echo "   (Or just restart your terminal)"
echo "--------------------------------------------------"