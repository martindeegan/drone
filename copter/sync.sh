#!/bin/sh

export RPI_ADDR="pi@10.0.0.147"
export DRONE_FOLDER=$1

echo '' > out.txt

nodemon -e rs,toml,json --watch ./ --exec 'rsync -uv -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh . $RPI_ADDR:/home/pi/$DRONE_FOLDER'

tail -f out.txt
