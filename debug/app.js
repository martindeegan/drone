var xVal = 0;
var yVal = 100;

var maxData = 500;

let p_data = [];
let i_data = [];
let d_data = [];
let power_total = [];

var options = {
    scaleStepWidth: -1,
    steppedLine: true,
}

var chart = new CanvasJS.Chart("chartContainer", {
    title: {
        text: "PID + power"
    },
    data: [{
        type: "line",
        dataPoints: power_total,
        color: "#F00",
        options: options
    }, {
        type: "line",
        dataPoints: p_data,
        color: "#00F",
        options: options
    }, {
        type: "line",
        dataPoints: i_data,
        options: options
    }, {
        type: "line",
        dataPoints: d_data,
        color: "#0F0",
        options: options
    }],
    //  options: {
    //        scales: {
    //              yAxes: [{
    //                  ticks: {
    //                      stepSize: -1,
    //                  }
    //              }]
    //          }
    //  }
});

var refresh_rate = 4;
var total_data = 0;

function addData(t, total, p, i, d) {
    p_data.push({
        x: t,
        y: p,
    });
    power_total.push({
        x: t,
        y: total,
    });
    i_data.push({
        x: t,
        y: i,
    });
    d_data.push({
        x: t,
        y: d,
    });
    truncate(p_data);
    truncate(i_data);
    truncate(d_data);
    truncate(power_total);

    if (total_data % refresh_rate == 0) {
        console.log("render");
        chart.render();
    }
}

function truncate(arr) {
    if (arr.length > maxData) {
        arr.splice(0, 1);
    }
}

var delegate = function(event) {
    var components = event.data.split(",");
    addData(parseInt(components[0]), parseFloat(components[1]), parseFloat(components[2]), parseFloat(components[3]), parseFloat(components[4]))
};

function setStatus(msg) {
    document.getElementById("connection-status").innerText = msg;
}

function reconnect() {
    var socket = new WebSocket("ws://10.0.0.213:27070", "rust-websocket");
    socket.onerror = function() {
        setStatus("Connection closed... wating for connection.");
        setTimeout( reconnect , 500);
    }
    socket.close = function() {
        console.log("Connection closed");
        setStatus("Connection closed... wating for connection.");
        setTimeout( reconnect , 500);
    }
    socket.onopen = function() {
         setStatus("Connected. Streaming data.");
        // Modify array in place.
        p_data.splice(0, p_data.length)
        i_data.splice(0, p_data.length)
        d_data.splice(0, p_data.length)
        power_total.splice(0, p_data.length)
    }
    
    
    socket.onmessage = delegate;
}

setStatus("Waiting for connection...");
reconnect();