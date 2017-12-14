#!/bin/sh

export RPI_ADDR="$1@$2"

echo '' > out.txt

nodemon -e rs,toml,json --watch ./ --exec 'rsync -uv -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' --exclude 'rls/' -e ssh . $RPI_ADDR:/home/pi/drone'

sudo tail -f out.txt
