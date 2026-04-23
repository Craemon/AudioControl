#!/bin/sh
set -e

echo "Removing AudioControl..."

sudo rm -f /usr/local/bin/audiocontrol
sudo rm -rf /usr/local/lib/audiocontrol

# optional: user config cleanup (commented for safety)
# rm -rf ~/.config/audiocontrol

echo "AudioControl removed successfully."