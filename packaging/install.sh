#!/bin/sh
set -e

VERSION="0.1.0"
BASE_URL="https://github.com/Craemon/AudioControl/releases/download/$VERSION"

INSTALL_DIR="/usr/local/lib/audiocontrol"

echo "Installing AudioControl $VERSION..."

# Python dependency (FIXED for venv + system Python)
echo "Checking Python dependency: pulsectl..."
if ! python3 -c "import pulsectl" 2>/dev/null; then
    echo "Installing pulsectl..."

    python3 -m pip install pulsectl || {
        echo "ERROR: failed to install pulsectl"
        echo "If you're in a venv, this is fine."
        echo "Otherwise try: sudo apt install python3-pulsectl"
        exit 1
    }
fi

# create dirs
sudo mkdir -p "$INSTALL_DIR"
sudo mkdir -p "$INSTALL_DIR/backend"
sudo mkdir -p "$INSTALL_DIR/config"

# download binary
sudo curl -L "$BASE_URL/audiocontrol" \
  -o "$INSTALL_DIR/audiocontrol"

# validate binary
if ! file "$INSTALL_DIR/audiocontrol" | grep -q "ELF"; then
    echo "ERROR: invalid binary downloaded"
    exit 1
fi

# backend
sudo curl -L "$BASE_URL/audio.py" \
  -o "$INSTALL_DIR/backend/audio.py"

# config
sudo curl -L "$BASE_URL/default.toml" \
  -o "$INSTALL_DIR/config/default.toml"

# permissions
sudo chmod +x "$INSTALL_DIR/audiocontrol"

# wrapper
cat <<'EOF' | sudo tee /usr/local/bin/audiocontrol > /dev/null
#!/bin/sh
exec /usr/local/lib/audiocontrol/audiocontrol "$@"
EOF

sudo chmod +x /usr/local/bin/audiocontrol

echo "Installed successfully. Run: audiocontrol"