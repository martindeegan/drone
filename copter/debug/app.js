const RANGE = 50;
const DOMAIN = 10000000; // 10 seconds.

function insertAfter(newNode, referenceNode) {
    referenceNode.parentNode.insertBefore(newNode, referenceNode.nextSibling);
}

class Line {
    constructor(canvas_id, color, name) {
        var canvas = document.getElementById(canvas_id);
        var ctx = canvas.getContext('2d');

        this.canvas = canvas;
        this.ctx = ctx;
        this.last_pt = null;
        this.start_time = 0;
        this.color = color;

        let key = document.createElement("div");
        key.innerText = name;
        key.style.backgroundColor = color;
        key.style.color = "White";
        key.style.display = "block";
        key.style.width = "100px";
        insertAfter(key, canvas);
    }

    range(y) {
        return (this.canvas.height / 2) + (y / RANGE * (this.canvas.height / 2))
    }

    domain(time) {
        return (time - this.start_time) / DOMAIN * this.canvas.width
    }

    addData(time, power) {

        if (!this.last_pt) {
            this.last_pt = {
                time: time,
                power: power,
            }
            return;
        }
        let ctx = this.ctx;
        ctx.beginPath();
        ctx.moveTo(this.domain(this.last_pt.time), this.range(this.last_pt.power));
        ctx.lineTo(this.domain(time), this.range(power));
        ctx.strokeStyle = this.color;
        ctx.lineWidth = 2;
        ctx.stroke();
        this.last_pt = {
            time: time,
            power: power,
        }
        if ((time - this.start_time) > DOMAIN) {
            this.last_pt = null;
            this.start_time = time;
            this.clear()
        }
    }

    clear() {
        let ctx = this.ctx;
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        ctx.lineWidth = 1;
        // draw lines at intervals of 10
        for (var i = -RANGE; i < RANGE; i += 10) {
            ctx.beginPath();
            ctx.moveTo(0, this.range(i));
            console.log("this.canvas.width: " + this.canvas.width);
            console.log("this.range(i): " + this.range(i));
            ctx.lineTo(this.canvas.width, this.range(i));
            ctx.strokeStyle = '#AAA';
            ctx.stroke();
        }

        ctx.beginPath();
        ctx.moveTo(0, this.range(0));
        ctx.lineTo(this.canvas.width, this.range(0));
        ctx.strokeStyle = 'black';
        ctx.stroke();
        this.last_pt = null;
    }
}

var canvas = document.getElementById("x-axis");
canvas.width = document.body.clientWidth;

let totalline = new Line("x-axis", "red", "total");
let pline = new Line("x-axis", "blue", "P");
let iline = new Line("x-axis", "green", "I");
let dline = new Line("x-axis", "orange", "D");

pline.clear();

function addData(t, total, p, i, d) {
    pline.addData(t, p);
    iline.addData(t, i);
    dline.addData(t, d);
    totalline.addData(t, total);
    gotData = true;
}

var delegate = function(event) {
    var components = event.data.split(",");
    addData(parseInt(components[0]), parseFloat(components[1]), parseFloat(components[2]), parseFloat(components[3]), parseFloat(components[4]))
};

function setStatus(msg) {
    document.getElementById("connection-status").innerText = msg;
}

let gotData = false;

function checkData() {
    if (gotData) {
        gotData = false;
        setTimeout(checkData, 1000);
    } else {
        reconnect();
    }
}

function reconnect() {
    var socket = new WebSocket("ws://10.0.0.213:27070", "drone-debug");
    socket.onerror = function() {
        setStatus("Connection closed... wating for connection.");
        setTimeout(reconnect, 500);
    }
    socket.close = function() {
        console.log("Connection closed");
        setStatus("Connection closed... wating for connection.");
        setTimeout(reconnect, 500);
    }
    socket.onopen = function() {
        setStatus("Connected. Streaming data.");
        pline.clear();
        dline.clear();
        totalline.clear();
        iline.clear();
        setTimeout(checkData, 1000);
    }
    socket.onmessage = delegate;
}

setStatus("Waiting for connection...");
reconnect();