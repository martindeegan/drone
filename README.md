# SafeFlight
This project is a pure rust implementation of a quadcopter.

## Features
1) PID loop for drone stabilization
<<<<<<< HEAD
2)
=======
2) Peer to peer communication
>>>>>>> 62e4ae1857f41fe8469eec35daf19860b2837112

## Todo
- [x] Move sensors to [i2cdev-sensors](https://github.com/martindeegan/i2cdev-sensors). Create a sensor manager that can be changed in config.json
- [ ] Catch and handle all matches.
- [ ] Refactor project
- [ ] Add several flight modes
- [ ] Dead reckoning local navigation
- [ ] GPS long distance navigation

## Teststed Hardware
### Micro Controller
- Raspberry Pi 3B (3B is needed for Wifi). Otherwise any Pi will do.

### Sensors
- LSM9DS0 Gyro, Accel, Mag
- L3GD20 Gyro
- BMP280 Barometer, Thermometer
- LSM303D Accel, Mag

### ESC - Electronic Speed Controllers
<<<<<<< HEAD
- BLHeli
=======
- BLHeli ESCs
>>>>>>> 62e4ae1857f41fe8469eec35daf19860b2837112

### Motors
- Crazepony Emax Mt2213 935kv Brushless Motor

## Installation
1) Install Rust https://www.rust-lang.org/en-US/install.html
2) Install Protobuf 3. For the Pi, you may need to install from source since there are no ARM releases.
3) Clone Safe Flight
3) Build with cargo:
~~~
cd copter
cargo build --release
~~~
4) Run with sudo:
~~~
sh run-release.sh
~~~

## Configuration
Modify copter.json to suit your specific configuration.
<<<<<<< HEAD
- Under motors, list the GPIO ports you used on your Raspberry Pi / Arduino
=======
- Under motors, list the GPIO ports you used on your Raspberry Pi
>>>>>>> 62e4ae1857f41fe8469eec35daf19860b2837112
