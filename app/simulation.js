import * as sim from 'lib-simulation-wasm';

export const renderCanvasViewport = (width = 600, height = 600) => {
  // create the simulation viewport
  const canvas = document
    .createElement('canvas');
  canvas.id = 'viewport';
  canvas
    .setAttribute('height', 600);
  canvas
    .setAttribute('width', 600);

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

  //* Trick for a sharper canvas render
  // scale up canvas's buffer to match the screen's pixel ratio || set the size of the canvas
  canvas.height = viewportHeight * scale;
  canvas.width = viewportWidth * scale;
  // scale down canvas's element || sets the resolution
  canvas.style.height = viewportHeight + 'px';
  canvas.style.width = viewportWidth + 'px';

  return {
    canvas,
    context
  }
}

export const createSimulation = () => {
  //* create a new Simulation object
  //TODO: get sim properties(methods for genetic algorithm etc) from the form
  // genetic algorithm methods
  // let selectionMethod = document.getElementById();
  // let crossoverMethod = document.getElementById();
  // let mutationMethod = document.getElementById();
  // sim generation length
  let generationLength = document
    .getElementById('input-generation-length').value || 2500;
  // no. of animals and foods
  let animals = document
    .getElementById('input-animals').value || 20;
  let foods = document
    .getElementById('input-foods').value || 30;

  // create a new instance of simulation
  const simulation = new sim
    .Simulation(generationLength, animals, foods);

  //TODO: render the entities(animals and food) with a nice transition before running the sim
  return simulation;
}

export const runSimulation = (simulation, canvas, context) => {
  //* Render the simulation and call the step method to progress through it
  //TODO: the main simulation loop
  setTimeout(() => {
    const viewportWidth = canvas.width;
    const viewportHeight = canvas.height;

    context
      .clearRect(0, 0, viewportWidth * 1.1, viewportHeight * 1.1);

    simulation
      .step();
    const world = simulation
      .world();

    // Render the food
    for (const food of world.food) {
      context
        .drawFood(
          food.x * viewportWidth,
          food.y * viewportHeight,
          0.005 * viewportWidth
        )
    }

    // Render the animals
    for (const animal of world.animals) {
      context
        .drawAnimal(
          animal.x * viewportWidth,
          animal.y * viewportHeight,
          animal.rotation,
          0.02 * viewportWidth
        );
    }

    window
      .requestAnimationFrame(() => runSimulation(simulation, canvas, context));
  }, 1000 / 60);
}

export const fastForwardGeneration = (simulation) => {
  //* fast forwards sim to the next generation
  //TODO: call the required method from the simulation lib
    simulation
      .fast_fwd();
}
