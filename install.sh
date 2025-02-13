#!/bin/bash

ENCRYPTED_TOKEN="U2FsdGVkX19DN2dacpjAsQXZ9LDmC5ed0Jk5P7bxRSI5lfVzxFges18V9w7FKmEf
UMZzYUz7ImVMzyjVWV2GYcYvLB977fKMZXaxIRJAH52CpQRTicmKHS37BJAd1DXg
qKycnI2Nn9Grq8XmAs81Rg=="
KEY="sigmasigma"

# Decrypt token
GITHUB_TOKEN=$(echo "$ENCRYPTED_TOKEN" | openssl enc -aes-256-cbc -md sha256 -a -d -k "$KEY" 2>/dev/null)

if [ $? -ne 0 ]; then
    echo "Error: Failed to decrypt token"
    exit 1
fi

ARCH=$(uname -m)
IS_PI_ZERO=false

if [ -f /sys/firmware/devicetree/base/model ]; then
    PI_MODEL=$(tr -d '\0' < /sys/firmware/devicetree/base/model)
    if [[ "$PI_MODEL" == *"Raspberry Pi Zero 2 W"* ]]; then
        IS_PI_ZERO=true
    fi
fi

if [ "$IS_PI_ZERO" = true ]; then

    echo "Detected Raspberry Pi Zero 2 W, downloading pre-compiled binary..."
    TEMP_DIR=$(mktemp -d)
    LATEST_RELEASE_URL=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
        https://api.github.com/repos/smartlinuxcoder/edgezone/actions/artifacts \
        | grep -o '"archive_download_url": "[^"]*' \
        | head -1 \
        | cut -d'"' -f4)
    
    cd "$TEMP_DIR"
    curl -L -H "Authorization: token $GITHUB_TOKEN" -o artifact.zip "$LATEST_RELEASE_URL"
    unzip artifact.zip
    sudo mv edgezone-node /usr/local/bin/
    cd - > /dev/null
    rm -rf "$TEMP_DIR"
    sudo chmod +x /usr/local/bin/edgezone-node
    BINARY_PATH=/usr/local/bin/edgezone-node
else

    if ! which cargo > /dev/null 2>&1; then
        echo "Cargo not found. Installing Rust and Cargo..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        source $HOME/.cargo/env
    else
        echo "Cargo is already installed, proceeding"
    fi

    cargo install edgezone-node
    BINARY_PATH=$(which edgezone-node)
fi

CURRENT_USER=$USER
WORKING_DIR=$HOME

cat << EOF | sudo tee /etc/systemd/system/edgezone-node.service
[Unit]
Description=EdgeZone Node Service
After=network.target

[Service]
Type=simple
User=$CURRENT_USER
WorkingDirectory=$WORKING_DIR
ExecStart=$BINARY_PATH
Restart=on-success
RestartSec=0
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=edgezone-node

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload

sudo systemctl enable edgezone-node
sudo systemctl start edgezone-node

echo "EdgeZone Node has been installed and service has been configured, started and enabled."
echo "Enjoy!"