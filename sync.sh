!#bin/bash

export RPI_ADDR="pi@10.0.0.213"
export DRONE_FOLDER=$1

echo '' > out.txt
nodemon -e rs,toml --watch Cargo.toml --watch src/ --exec 'sh sync-src.sh' >> out.txt &
nodemon -e rs,toml --watch protos --exec 'sh sync-proto.sh' >> out.txt &
nodemon -e rs,toml --watch sensors/src --exec 'sh sync-sensors.sh' >> out.txt &
nodemon -e rs,toml --watch debug/src --exec 'sh sync-debug.sh' >> out.txt &
nodemon -e json --watch config.json --exec 'sh sync-config.sh' >> out.txt &

tail -f out.txt
