#!/bin/sh
set -e

VERSION=$(curl -s https://api.github.com/repos/Craemon/AudioControl/releases/latest \
  | grep tag_name | cut -d '"' -f 4)

BASE_URL="https://github.com/Craemon/AudioControl/releases/download/$VERSION"

echo "Updating to $VERSION..."

# update binary
sudo curl -L "$BASE_URL/audiocontrol" \
  -o /usr/local/lib/audiocontrol/audiocontrol

# update python backend
sudo curl -L "$BASE_URL/audio.py" \
  -o /usr/local/lib/audiocontrol/backend/audio.py

# update default config (optional but recommended)
sudo curl -L "$BASE_URL/default.toml" \
  -o /usr/local/lib/audiocontrol/config/default.toml

sudo chmod +x /usr/local/lib/audiocontrol/audiocontrol

echo "Update complete."