# SafeFlight
This project is a pure rust implementation of a quadcopter.

##### Tuned PID:
[![Tuned PID](http://img.youtube.com/vi/ZvyTtImR9pY/0.jpg)](https://www.youtube.com/watch?v=ZvyTtImR9pY)
##### First Flight:
[![First Flight!](http://img.youtube.com/vi/_O6T4tCpLQM/0.jpg)](https://www.youtube.com/watch?v=_O6T4tCpLQM)

## Features
1) 19 State Kalman Filter
2) PID loop for drone stabilization
3) 3D graphical visualizer of quad state

## Todo
- [x] Move sensors to [i2cdev-sensors](https://github.com/martindeegan/i2cdev-sensors). Create a sensor manager that can be changed in config.json
- [x] Refactor project
- [x] Convert to an Extended Kalman Filter to track location and attitude.
- [ ] GPS long distance navigation.

## Teststed Hardware
### Micro Controller
- Raspberry Pi 3B (3B is needed for Wifi). Otherwise any Pi will do.

### Sensors
- LSM9DS0 Gyro, Accel, Mag
- L3GD20 Gyro
- BMP280 Barometer, Thermometer
- LSM303D Accel, Mag

### ESC - Electronic Speed Controllers
- BLHeli ESCs

### Motors
- Crazepony Emax Mt2213 935kv Brushless Motor

## Installation
1) Install Rust https://www.rust-lang.org/en-US/install.html
2) Install Protobuf 3. For the Pi, you may need to install from source since there are no ARM releases.
3) Clone the repo
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
- Under motors, list the GPIO ports you used on your Raspberry Pi
