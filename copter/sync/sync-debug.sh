rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh debug/ $RPI_ADDR:/home/pi/$DRONE_FOLDER/debug
