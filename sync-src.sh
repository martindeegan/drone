rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh src/ pi@10.0.0.213:/home/pi/drone/src
rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh Cargo.toml pi@10.0.0.213:/home/pi/drone/Cargo.toml
rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh config.json pi@10.0.0.213:/home/pi/drone/config.json

