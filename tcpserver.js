const dgram = require('dgram');
const drone_server = dgram.createSocket('udp4');
const desktop_server = dgram.createSocket('udp4');
var drone_addr;
var drone_port;
var desktop_addr;
var desktop_port;

function send_info() {
  drone_server.send(desktop_addr + ':' + desktop_port.toString(), drone_port, drone_addr);
  desktop_server.send(drone_addr + ':' + drone_port.toString(), desktop_port, desktop_addr);
  drone_addr = null;
  drone_port = null;
  desktop_addr = null;
  desktop_port = null;
}

desktop_server.on('message', (msg, rinfo) => {
  console.log('Connection from desktop at', rinfo.address, rinfo.port);
  desktop_addr = rinfo.address;
  desktop_port = rinfo.port;
  if(drone_addr) {
    send_info();
  }
});

desktop_server.on('listening', () => {
  console.log('listening for desktop');
});

drone_server.on('message', (msg, rinfo) => {
  console.log('Connection from drone at', rinfo.address, rinfo.port);
  drone_addr = rinfo.address;
  drone_port = rinfo.port;
  if(desktop_addr) {
    send_info();
  }
});

drone_server.on('listening', () => {
  console.log('listening for drone');
});



drone_server.bind(6060);
desktop_server.bind(7070);
