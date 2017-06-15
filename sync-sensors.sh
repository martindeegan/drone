rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh sensors/ pi@10.0.0.213:/home/pi/drone/sensors
