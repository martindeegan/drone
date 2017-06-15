rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh sensors/ $RPI_ADDR:/home/pi/$DRONE_FOLDER/sensors
