#!/bin/sh

curl -sL https://deb.nodesource.com/setup_6.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo npm install npm -g
npm install -g nodemon

git submodule update --init