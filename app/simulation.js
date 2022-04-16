import * as sim from 'lib-simulation-wasm';

export const createSimulation = () => {
  //* create a new Simulation object
  //TODO: get sim properties(methods for genetic algorithm etc) from the form
  // genetic algorithm methods
  // let selectionMethod = document.getElementById();
  // let crossoverMethod = document.getElementById();
  // let mutationMethod = document.getElementById();
  // sim generation length
  let generationLength = document
    .getElementById('input-generation-length') || 2500;
  // no. of animals and foods
  let animals = document
    .getElementById('input-animals') || 20;
  let foods = document
    .getElementById('input-foods') || 30;

  // create a new instance of simulation
  const simulation = new sim
    .Simulation(generationLength, animals, foods);

  //TODO: render the entities(animals and food) with a nice transition before running the sim
  return simulation;
}

export const runSimulation = (simulation, canvas, context) => {
  //* Render the simulation and call the step method to progress through it
  //TODO: the main simulation loop
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
      .fillCircle(
        food.x * viewportWidth,
        food.y * viewportHeight,
        0.005 * viewportWidth
      )
  }

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

  window
    .requestAnimationFrame(() => runSimulation(simulation, canvas, context));
}

const fastForwardGeneration = (simulation) => {
  //* fast forwards sim to the next generation
  //TODO: call the required method from the simulation lib
  //TODO: slow down the simulation to a reasonable speed
}
