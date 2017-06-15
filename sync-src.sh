rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh src/ $RPI_ADDR:/home/pi/$DRONE_FOLDER/src
rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh Cargo.toml $RPI_ADDR:/home/pi/$DRONE_FOLDER/Cargo.toml
rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh build.rs $RPI_ADDR:/home/pi/$DRONE_FOLDER/build.rs

