import * as sim from 'lib-simulation-wasm';

// create a new simulation
const simulation = new sim.Simulation();
// generate and world and randomly populate it with animals and food
const world = simulation.world();

// HTML Canvas setup
const canvas = document
  .getElementById('viewport');
const context = canvas
  .getContext('2d');

const scale = window.devicePixelRatio || 1;

const viewportHeight = canvas.height;
const viewportWidth =  canvas.width;

//* Trick for a sharper canvas render
// scale up canvas's buffer to match the screen's pixel ratio
canvas.height = viewportHeight * scale;
canvas.width = viewportWidth * scale;
// scale down canvas's element
canvas.style.height = viewportHeight + 'px';
canvas.style.width = viewportWidth + 'px';
//* #################################

canvas.setAttribute('height', viewportHeight * scale)
canvas.setAttribute('height', viewportHeight * scale)

CanvasRenderingContext2D.prototype.fillTriangle = function (x, y, rotation, side) {
  this.beginPath();
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

  this.fillStyle = 'rgb(0, 0, 0)';
  this.fill();
}

CanvasRenderingContext2D.prototype.fillCircle = function (x, y, radius) {
  this.beginPath();
  this.arc(x, y, radius, 0, 2 * Math.PI, false);

  this.fillStyle = 'rgb(0, 0, 0)';
  this.fill();
}

context.fillStyle = 'rgb(0, 0, 0)';

// Render the animals
for (const animal of world.animals) {
  context
    .fillTriangle(
      animal.x * viewportWidth,
      animal.y * viewportHeight,
      animal.rotation,
      0.02 * viewportWidth
    );
}

// Render the food
for (const food of world.food) {
  context
    .fillCircle(
      food.x * viewportWidth,
      food.y * viewportHeight,
      0.005 * viewportWidth
    )
}
