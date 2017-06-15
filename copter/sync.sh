!#bin/bash

export RPI_ADDR="pi@10.0.0.213"
export DRONE_FOLDER=$1

forever start -c nodemon -e rs,toml --watch Cargo.toml --watch src/ --exec 'sh sync/sync-src.sh'
forever start -c nodemon -e rs,toml --watch protos --watch protos/Cargo.toml --exec 'sh sync/sync-proto.sh'
forever start -c nodemon -e rs,toml --watch sensors/src --watch sensors/Cargo.toml --exec 'sh sync/sync-sensors.sh'
forever start -c nodemon -e rs,toml --watch debug/src --watch debug/Cargo.toml --exec 'sh sync/sync-debug.sh'
nodemon -e json --watch config.json --exec 'sh sync/sync-config.sh'

forever stopall
