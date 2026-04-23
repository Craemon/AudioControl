#!/bin/sh
set -e

LATEST=$(curl -s https://api.github.com/repos/Craemon/AudioControl/releases/latest \
  | grep tag_name | cut -d '"' -f 4)

BASE_URL="https://github.com/Craemon/AudioControl/releases/download/$LATEST"
INSTALL_DIR="/usr/local/lib/audiocontrol"

echo "Updating to $LATEST..."

# Python dependency check (FIXED)
echo "Checking Python dependency: pulsectl..."
if ! python3 -c "import pulsectl" 2>/dev/null; then
    echo "Installing pulsectl..."

    python3 -m pip install pulsectl || {
        echo "ERROR: failed to install pulsectl"
        echo "Try system package: sudo apt install python3-pulsectl"
        exit 1
    }
fi

# update binary
sudo curl -L "$BASE_URL/audiocontrol" \
  -o "$INSTALL_DIR/audiocontrol"

# validate binary
if ! file "$INSTALL_DIR/audiocontrol" | grep -q "ELF"; then
    echo "ERROR: invalid binary downloaded during update"
    exit 1
fi

# update backend
sudo curl -L "$BASE_URL/audio.py" \
  -o "$INSTALL_DIR/backend/audio.py"

# update config
sudo curl -L "$BASE_URL/default.toml" \
  -o "$INSTALL_DIR/config/default.toml"

sudo chmod +x "$INSTALL_DIR/audiocontrol"

echo "Update complete."