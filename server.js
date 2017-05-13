const express = require('express');
var app = express();

var drone_info = '';
var desktop_info = '';

//Information times out after a minute
var drone_interval;
var desktop_interval;

app.get('/drone', function(req, res) {
  console.log('Requesting drone information');
  res.send(drone_info);
});

app.post('/drone', function(req, res) {
  if(drone_interval) {
    clearInterval(drone_interval);
  }
  console.log('Posting drone information');
  drone_info = req.ip;
  console.log(drone_info);
  drone_interval = setInterval(function() {
    drone_info = '';
  }, 60000);
  res.send('');
});

app.get('/desktop', function(req, res) {
  console.log('Requesting desktop information');
  res.send(desktop_info);
});

app.post('/desktop', function(req, res) {
  if(desktop_interval) {
    clearInterval(desktop_interval);
  }
  console.log('Posting desktop information');
  desktop_info = req.ip;
  console.log(desktop_info);
  desktop_interval = setInterval(function() {
    desktop_info = '';
  }, 60000);
  res.send('');
});

var port = parseInt(process.argv[2]);
app.listen(port);
