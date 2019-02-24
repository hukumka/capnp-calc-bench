 #! /bin/bash

set -x
set -e

cargo build --release
target/release/calculator server 127.0.0.1:6569 &
SERVER=$!
sleep 1
#target/release/calculator bench
cargo bench

kill $SERVER
