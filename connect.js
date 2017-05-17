const dgram = require('dgram');

const socket = dgram.createSocket('udp4');

socket.on('message', (msg, rinfo) => {
  console.log(msg.toString());
  var parts = msg.toString().split(':');
  var len = parts.length;
  var ip = parts[len - 2];
  var port = parseInt(parts[len - 1]);
  socket.send('Sup', port, ip);
});

socket.send('World', 7070, '127.0.0.1');
