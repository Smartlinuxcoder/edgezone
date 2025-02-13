#!/bin/bash

if ! command -v cargo &> /dev/null; then
    echo "Cargo not found. Installing Rust and Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    echo "Cargo is already installed, proceeding"
fi

cargo install edgezone-node

CURRENT_USER=$USER
BINARY_PATH=$(which edgezone-node)
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