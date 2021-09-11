#!/bin/bash

cargo build --release
sudo cp ./target/release/xtop /usr/bin/