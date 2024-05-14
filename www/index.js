import * as sim from "simulation-wasm";

const simulation = new sim.Simulation;
const world = simulation.world();

const viewport = document.getElementById("viewport");
const vh = viewport.height;
const vw = viewport.width;
const viewportScale = window.devicePixelRatio || 1;

viewport.width = vw * viewportScale;
viewport.height = vh * viewportScale;

viewport.style.height = vh + 'px';
viewport.style.width = vw + 'px';

CanvasRenderingContext2D.prototype.drawTriangle = 
 function(x,y, size, rotation) {
  this.beginPath();
  this.moveTo(
   x - Math.sin(rotation) * size * 1.5, 
   y + Math.cos(rotation) * size * 1.5
  );
  this.lineTo(
   x - Math.sin(rotation + 2.0/3.0 * Math.PI) * size, 
   y + Math.cos(rotation + 2.0/3.0 * Math.PI) * size
  );
  this.lineTo(
   x - Math.sin(rotation + 4.0/3.0 * Math.PI) * size, 
   y + Math.cos(rotation + 4.0/3.0 * Math.PI) * size
  );
  this.lineTo(
   x - Math.sin(rotation) * size * 1.5, 
   y + Math.cos(rotation) * size * 1.5
  );

  this.stroke()
  this.fillStyle = 'rgb(222,222,222)';
  this.fill();
 }

CanvasRenderingContext2D.prototype.drawCircle =
 function (x, y, radius) {
  this.beginPath();

  this.arc(x, y, radius, 0, 2.0 * Math.PI);
  this.fillStyle = 'rgb(0, 255, 125)';
  this.fill();
 }

const ctxt = viewport.getContext("2d");
ctxt.scale(viewportScale, viewportScale);

ctxt.fillStyle = 'rgb(255, 0, 0)';

for (const animal of world.animals) {
 // ctxt.fillRect(animal.x * vw,animal.y * vh, 15, 15);
 ctxt.drawTriangle(animal.x*vw, animal.y*vh, 0.01*vw, animal.rotation);
}

function redraw() {
 ctxt.clearRect(0, 0, vw, vh);

 simulation.step();
 const world = simulation.world();

 for (const food of world.foods) {
  ctxt.drawCircle(
  food.x * vw,
   food.y * vh,
   (0.01 / 2.0) * vw,
  )
 } 
  
 for (const animal of world.animals) {
  ctxt.drawTriangle(
   animal.x * vw,
   animal.y * vh,
   0.01 * vw,
   animal.rotation,
  );
 }

 requestAnimationFrame(redraw);
}

redraw();
