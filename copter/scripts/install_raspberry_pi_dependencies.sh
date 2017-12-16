#!/bin/sh

curl https://sh.rustup.rs -sSf | sh
sudo apt-get install libssl-dev
sudo apt-get install pigpio python-pigpio python3-pigpio

git submodule update --init
