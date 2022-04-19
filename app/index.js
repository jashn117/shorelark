import {
  renderCanvasViewport,
  createSimulation,
  runSimulation,
  fastForwardGeneration,
} from './simulation';

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

const {canvas, context} = renderCanvasViewport();

document
  .getElementById('start-sim-btn')
  .onclick = () => {
    // create the start the simulation
    const sim = createSimulation();
    runSimulation(sim, canvas, context);

    document
      .getElementById('fast-fwd-btn')
      .onclick = () => fastForwardGeneration(sim);
  }
