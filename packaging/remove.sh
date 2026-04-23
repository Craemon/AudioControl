#!/bin/sh
set -e

echo "Removing AudioControl..."

sudo rm -f /usr/local/bin/audiocontrol
sudo rm -rf /usr/local/lib/audiocontrol

echo "AudioControl removed."