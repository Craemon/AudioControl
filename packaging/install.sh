#!/bin/sh
set -e

VERSION="0.1.0"
BASE_URL="https://github.com/Craemon/AudioControl/releases/download/$VERSION"

echo "Installing AudioControl $VERSION..."

# install dirs
sudo mkdir -p /usr/local/lib/audiocontrol
sudo mkdir -p /usr/local/lib/audiocontrol/backend
sudo mkdir -p /usr/local/lib/audiocontrol/config

# binary
sudo curl -L "$BASE_URL/audiocontrol" \
  -o /usr/local/lib/audiocontrol/audiocontrol

# python backend
sudo curl -L "$BASE_URL/audio.py" \
  -o /usr/local/lib/audiocontrol/backend/audio.py

# default config
sudo curl -L "$BASE_URL/default.toml" \
  -o /usr/local/lib/audiocontrol/config/default.toml

sudo chmod +x /usr/local/lib/audiocontrol/audiocontrol

# wrapper
cat <<'EOF' | sudo tee /usr/local/bin/audiocontrol > /dev/null
#!/bin/sh
exec /usr/local/lib/audiocontrol/audiocontrol "$@"
EOF

sudo chmod +x /usr/local/bin/audiocontrol

echo "Installed. Run: audiocontrol"