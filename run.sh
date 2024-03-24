#!/bin/sh
# Ensure Cargo is in the PATH
export PATH="$HOME/.cargo/bin:$PATH"

# This script is used to run the thunder program
cargo b --release

# Assign capabilities to the thunder executable
sudo setcap cap_net_admin=eip target/release/thunder

# Run thunder in the background
./target/release/thunder &
pid=$!

# Configure thunder0 interface
# Check if the IP is already assigned and remove it if so
if ip addr show thunder0 | grep -q "192.168.1.0/24"; then
    sudo ip addr del 192.168.1.0/24 dev thunder0
fi

if ip tuntap show | grep -qw thunder0; then
    sudo ip tuntap del mode tun dev thunder0
fi

# Add the IP address to thunder0 interface
sudo ip addr add 192.168.1.0/24 dev thunder0

# Bring the interface up
sudo ip link set dev thunder0 up

# Wait for the background process to finish
wait $pid
