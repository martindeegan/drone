!#bin/bash

export RPI_ADDR="pi@10.0.0.213"
export DRONE_FOLDER="drone"

forever start -c nodemon -e rs,toml --watch Cargo.toml --watch src/ --exec 'sh sync-src.sh'
forever start -c nodemon -e rs,toml --watch protos/src --exec 'sh sync-proto.sh'
forever start -c nodemon -e rs,toml --watch sensors/src/ --exec 'sh sync-sensors.sh'
nodemon -e json --watch config.json --exec 'sh sync-config.sh'

forever stopall
