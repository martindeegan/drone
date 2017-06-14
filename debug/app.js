var xVal = 0;
var yVal = 100;

var maxData = 1000;

let p_data = [];
let i_data = [];
let d_data = [];
let power_total = [];

var chart = new CanvasJS.Chart("chartContainer", {
  title: {
    text: "PID + power"
  },
  data: [
  {
    type: "line",
    dataPoints: power_total,
  },
  {
    type: "line",
    dataPoints: p_data
  },
  {
    type: "line",
    dataPoints: i_data
  },
  {
    type: "line",
    dataPoints: d_data,
    // color: "#100"
  }]
});


var counter = 0;
function addData(t, total, p, i, d) {
  p_data.unshift({
    x: t,
    y: p,
  });
  power_total.unshift({
    x: t,
    y: p,
  });
  i_data.unshift({
    x: t,
    y: i,
  });
  d_data.unshift({
    x: t,
    y: d,
  });
  truncate(p_data);
  truncate(i_data);
  truncate(d_data);
  truncate(power_total);
  if (counter == 5) {
    counter = 0;
    chart.render();
  }
  counter++;
  // var output = document.getElementById("output");
  // var row = document.createElement("div");
  // row.innerText = p + "," + i + "," + d;
  // output.appendChild(row);
}

function truncate(arr) {
  if (arr.length > maxData) {
    arr.pop();
  }
}


var socket = new WebSocket("ws://10.0.0.213:27070", "rust-websocket");
socket.onmessage = function (event) {
  var components = event.data.split(",");
  // console.log("got data: "?+ components);
  addData(parseInt(components[0]), parseFloat(components[2]), parseFloat(components[3]), parseFloat(components[4]))
};
