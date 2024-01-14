#!/usr/bin/zsh

cargo b --release
sudo setcap cap_net_admin=eip ./target/release/tcp

./target/release/tcp &

pid=$!

sudo ip addr add 192.168.1.2/24 dev tcp
sudo ip link set up dev tcp

trap "kill $pid" INT TERM

wait $pid
