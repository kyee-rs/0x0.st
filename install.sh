#!/bin/sh
cd /bin
if [ -f /bin/0x0 ]; then
    echo "0x0 already installed"
    echo "reinstalling..."
    sudo rm /bin/0x0
    sudo wget https://github.com/voxelin/0x0.st/releases/latest/download/0x0 -q --show-progress
    sudo chmod +x 0x0
    echo "0x0 reinstalled successfully!"
    exit 1
fi

sudo wget https://github.com/voxelin/0x0.st/releases/latest/download/0x0 -q --show-progress
sudo chmod +x 0x0

if [ -f /bin/0x0 ]; then
    echo "0x0 installed successfully!"
    exit 0
fi;

echo "0x0 installation failed!"