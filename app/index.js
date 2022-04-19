import {createSimulation, runSimulation, fastForwardGeneration} from './simulation';

// create the simulation viewport
const canvas = document
  .createElement('canvas');
canvas.id = 'viewport';
canvas.height = 800;
canvas.width = 800;

const fastFwdBtn = document
  .createElement('button');
fastFwdBtn.type = 'button';
fastFwdBtn.id = 'fast-fwd-btn';
fastFwdBtn.innerHTML = '&#x23e9; Generation';

document.body
  .appendChild(canvas);

document.body
  .appendChild(fastFwdBtn);

const context = canvas
  .getContext('2d');

const scale = window.devicePixelRatio || 1;

const viewportHeight = canvas.height;
const viewportWidth =  canvas.width;

//TODO: fix the canvas element's size/resolution
//* Trick for a sharper canvas render
// scale up canvas's buffer to match the screen's pixel ratio || set the size of the canvas
canvas.height = viewportHeight * scale;
canvas.width = viewportWidth * scale;
// scale down canvas's element || sets the resolution
canvas.style.height = viewportHeight + 'px';
canvas.style.width = viewportWidth + 'px';
//* #################################

//* Method to draw the animal entity
CanvasRenderingContext2D.prototype.drawAnimal = function (x, y, rotation, side) {
  this.beginPath();
  this.lineWidth = 3;
  this.moveTo(
    x + Math.cos(rotation) * side * 1.5,
    y + Math.sin(rotation) * side * 1.5
  );
  this.lineTo(
    x + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * side,
    y + Math.sin(rotation + (2.0 / 3.0) * Math.PI) * side
  );
  this.lineTo(
    x + Math.cos(rotation + (4.0 / 3.0) * Math.PI) * side,
    y + Math.sin(rotation + (4.0 / 3.0) * Math.PI) * side
  );
  this.lineTo(
    x + Math.cos(rotation) * side * 1.5,
    y + Math.sin(rotation) * side * 1.5
  );

  this.strokeStyle = 'rgb(232, 106, 146)';
  this.stroke();
  // this.fillStyle = 'rgb(232, 106, 146)';
  // this.fill();
}

//* Method to draw food entity 
CanvasRenderingContext2D.prototype.drawFood = function (x, y, radius) {
  this.beginPath();
  this.arc(x, y, radius, 0, 2 * Math.PI, false);

  this.fillStyle = 'rgb(247, 231, 51)';
  this.fill();
}

// create the start the simulation
const sim = createSimulation();

document
  .getElementById('fast-fwd-btn')
  .onclick = () => fastForwardGeneration(sim);

runSimulation(sim, canvas, context);
