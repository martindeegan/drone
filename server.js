const nodersa = require('node-rsa');
const randomstring = require('randomstring');
const net = require('net');
const fs = require('fs');

//Need to extablish TCP connection with both drone and desktop
var connected_drone = false;
var connected_desktop = false;

const drone_pub_key;
const desktop_pub_key;

fs.readFile('drone_key.pub', function(err, data) {
  if(err) {
    throw err;
  }
  else {
    drone_pub_key = nodersa(data);
  }
});

fs.readFile('desktop_key.pub', function(err, data) {
  if(err) {
    throw err;
  }
  else {
    desktop_pub_key = nodersa(data);
  }
});

class ServerType {
  ServerType() {
    
  }
};

var drone_info = {};
var desktop_info = {};

var droneServer;
var desktopServer;

var droneSocket;
var desktopSocket;

function initiateP2P(){
  if(droneSocket && desktopSocket) {

  }
}

droneServer = net.createServer((socket) => {
  let randStr = randomstring(100);
  let state = 'unauthenticated';
  socket.on('connect', function(){
    console.log('Connected');
    socket.write(drone_pub_key.encrypt(randStr));
  });

  socket.on('data', function(data) {
    switch(state) {
      case 'unauthenticated': {
        if(data === randStr) {
          console.log('Authenticated', socket.remoteAddress);
          state = 'authenticated'
          socket.write(1);
        }
        else {
          console.log('Rejected', socket.remoteAddress);
          socket.write(0);
          socket.end();
        }
        break;
      }
      case 'authenticated': {
        state = 'first';
        drone_info.public = data;
        socket.write(1);
        break;
      }
      case 'first': {
        state = '';
        drone_info.private = data;
        connected_drone = true;
        socket.write(1);
        droneSocket = socket;
        if(connected_desktop) {
          initiateP2P();
        }
        break;
      }
      default: {
        socket.write(1);
      }
    }
  });

  socket.on('close', function(){
    connected_drone = false;
    state = 'unauthenticated';
    if(!droneServer.listening) {
      droneServer.listen(6060);
    }
  });
}).on('error', (err) => {
  throw err;
});

droneServer.listen(6060, () => {
  console.log('Drone server listening on', droneServer.address());
});

desktopServer = net.createServer((socket) => {
  let randStr = randomstring(100);
  let state = 'unauthenticated';
  socket.on('connect', function(){
    console.log('Connected');
    socket.write(desktop_pub_key.encrypt(randStr));
  });

  socket.on('data', function(data) {
    switch(state) {
      case 'unauthenticated': {
        if(data === randStr) {
          console.log('Authenticated', socket.remoteAddress);
          state = 'authenticated'
          socket.write(1);
        }
        else {
          console.log('Rejected', socket.remoteAddress);
          socket.write(0);
          socket.end();
        }
        break;
      }
      case 'authenticated': {
        state = 'first';
        desktop_info.public = data;
        socket.write(1);
        break;
      }
      case 'first': {
        desktop_info.private = data;
        connected_desktop = true;
        socket.write(1);
        desktopSocket = socket;
        if(connected_drone) {
          initiateP2P();
        }
        break;
      }
      default: {
        socket.write(1);
      }
    }
  });

  socket.on('close', function(){
    connected_desktop = false;
    state = 'unauthenticated';
    if(!desktopServer.listening) {
      desktopServer.listen(7070);
    }
  });
}).on('error', (err) => {
  throw err;
});

desktopServer.listen(7070, () => {
  console.log('Desktop server listening on', server.address());
});
