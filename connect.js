const dgram = require('dgram');
const socket = dgram.createSocket('udp4');

var controllerAddr;
var controllerPort;

var droneAddr;
var dronePort;

function sendEndpoints() {
  let controllerEP = controllerAddr + ':' + controllerPort.toString();
  let droneEP = droneAddr + ':' + dronePort.toString();

  socket.send(controllerEP, dronePort, droneAddr);
  socket.send(droneEP, controllerPort, controllerAddr);

  controllerAddr = null;
  controllerPort = null;
  droneAddr = null;
  dronePort = null;
}

socket.on('message', (msg, rinfo) => {
  console.log('Message origin: ' + rinfo.address + ':' + rinfo.port.toString());
  if(msg.length > 1) {
    console.log(msg.toString());
    if(msg.toString() === 'controller') {
      //Controller ping 
      controllerAddr = rinfo.address;
      controllerPort = rinfo.port;

      socket.send("Pong", controllerPort, controllerAddr);
    }
    else if(msg.toString() === 'drone') {
      //Drone ping
      droneAddr = rinfo.address;
      dronePort = rinfo.port;

      socket.send("Pong", dronePort, droneAddr);
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