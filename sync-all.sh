rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh ./ pi@10.0.0.213:/home/pi/drone/
