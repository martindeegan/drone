const dgram = require('dgram');
const socket = dgram.createSocket('udp4');

var controllerAddr;
var controllerPort;
var controllerTimeout;

var droneAddr;
var dronePort;
var droneTimeout;

function clearController() {
  controllerAddr = null;
  controllerPort = null;
}

function clearDrone() {
  droneAddr = null;
  dronePort = null;
}

function sendEndpoints() {
  let controllerEP = controllerAddr + ':' + controllerPort.toString();
  let droneEP = droneAddr + ':' + dronePort.toString();

  socket.send(controllerAddr, dronePort, droneAddr);
  socket.send(droneEP, controllerPort, controllerAddr);
  socket.send(controllerPort.toString(), dronePort, droneAddr);

  if(controllerTimeout) {
        clearTimeout(controllerTimeout);
        controllerTimeout = null;
  }

  if(droneTimeout) {
        clearTimeout(droneTimeout);
        droneTimeout = null;
  }
  clearController();
  clearDrone();
}

socket.on('message', (msg, rinfo) => {
  if(msg.length > 1) {
    if(msg.toString() === 'controller') {
      console.log('Controller connected');
      clearTimeout(controllerTimeout);
      controllerAddr = rinfo.address;
      controllerPort = rinfo.port;
      controllerTimeout = setTimeout(clearController, 5000);

      socket.send("Pong", controllerPort, controllerAddr);
    } else if(msg.toString() === 'drone') {
      console.log('Drone connected');
      clearTimeout(droneTimeout);
      droneAddr = rinfo.address;
      dronePort = rinfo.port;
      droneTimeout = setTimeout(clearDrone, 5000);
 
      socket.send("Pong", dronePort, droneAddr);
    }
  } else {
    switch(rinfo.address) {
      case controllerAddr: 
        //Controller ping
        clearTimeout(controllerTimeout);
        controllerTimeout = setTimeout(clearController, 5000)
        break;
      case droneAddr: 
        //Drone ping
        clearTimeout(droneTimeout);
        droneTimeout = setTimeout(clearDrone, 5000);
        break;
    }
  }

  if(droneAddr && controllerAddr) {
    sendEndpoints();
  }

});

socket.on('listen', () => {
  console.log('Listening');
});

socket.bind(7070);