#!/bin/sh

export RPI_ADDR="$1"

echo '' > out.txt

nodemon -e rs,toml,json --watch ./ --exec 'rsync -uv -r --exclude 'target/' --exclude 'visualizer/' --exclude 'configuration/calibration.toml' --exclude '.git/' --exclude '.idea/' --exclude 'rls/' -e ssh . $RPI_ADDR:/home/pi/drone/copter'

sudo tail -f out.txt
