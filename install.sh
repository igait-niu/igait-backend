#!/bin/bash
# Installation script for iGait Pipeline
# Run from anywhere: /lstr/sahara/zwlab/jw/igait-pipeline/install.sh

set -e

WORKSPACE_ROOT="/lstr/sahara/zwlab/jw/igait-pipeline"
MODULE_FILE="$WORKSPACE_ROOT/modulefiles/igait-pipeline"
USER_MODULE_DIR="$HOME/.local/share/Modules/modulefiles/igait"

echo "=== iGait Pipeline Installation ==="
echo ""

# Check if the binary exists
if [ ! -f "$WORKSPACE_ROOT/target/release/igait-pipeline" ]; then
    echo "ERROR: Binary not found at $WORKSPACE_ROOT/target/release/igait-pipeline"
    echo "Please contact the system administrator."
    exit 1
fi
echo "✓ Found igait-pipeline binary"
echo ""

# Install module
echo "Installing module for your account..."
mkdir -p "$USER_MODULE_DIR"
cp "$MODULE_FILE" "$USER_MODULE_DIR/default"
echo "✓ Module installed to $USER_MODULE_DIR/default"
echo ""

# Automatically add to .bashrc if not already present
BASHRC="$HOME/.bashrc"
MODULE_USE_LINE="module use \$HOME/.local/share/Modules/modulefiles"

if [ -f "$BASHRC" ]; then
    if grep -q "module use.*\.local/share/Modules/modulefiles" "$BASHRC"; then
        echo "✓ Module path already in ~/.bashrc"
    else
        echo "" >> "$BASHRC"
        echo "# iGait Pipeline - Added by install script" >> "$BASHRC"
        echo "$MODULE_USE_LINE" >> "$BASHRC"
        echo "✓ Added module path to ~/.bashrc"
    fi
else
    echo "$MODULE_USE_LINE" > "$BASHRC"
    echo "✓ Created ~/.bashrc with module path"
fi

echo ""
echo "=== Installation Complete! ==="
echo ""
echo "To start using the pipeline:"
echo "  1. Run: source ~/.bashrc"
echo "  2. OR start a new terminal session"
echo ""
echo "Then run:"
echo "  module load igait"
echo "  igait-pipeline --help"
echo ""
