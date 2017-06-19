#!/bin/sh

export RPI_ADDR="pi@10.0.0.213"
export DRONE_FOLDER=$1

echo '' > out.txt

nodemon -e rs,toml,json --watch ./ --exec 'rsync -uv -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh . $RPI_ADDR:/home/pi/$DRONE_FOLDER'

#nodemon -e rs,toml --watch src/ --exec 'sh sync/sync-src.sh' >> out.txt &
#nodemon -e rs,toml --watch protos --exec 'sh sync/sync-proto.sh' >> out.txt &
#nodemon -e rs,toml --watch sensors/src --exec 'sh sync/sync-sensors.sh' >> out.txt &
#nodemon -e rs,toml --watch debug/src --exec 'sh sync/sync-debug.sh' >> out.txt &
#nodemon -e json --watch config.json --exec 'sh sync/sync-config.sh' >> out.txt &

tail -f out.txt
