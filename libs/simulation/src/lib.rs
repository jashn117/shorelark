use rand::{Rng, RngCore};
use nalgebra as na;
use lib_genetic_algorithm as ga;
use std::f32::consts::FRAC_PI_8;

pub mod world;
mod eye;
mod brain;
mod individual;

// CONSTANTS
const MIN_SPEED: f32 = 0.001;
const MAX_SPEED: f32 = 0.005;
const LIN_ACCELERATION: f32 = 0.2;
const ROT_ACCELERATION: f32 = FRAC_PI_8;

pub struct Simulation {
    world: world::World,
    genetic_algo: ga::GeneticAlgorithm<
        ga::selection::RoulleteWheelSelection,
        ga::crossover::UniformCrossover>,
    age: usize,
    generation_length: usize,
}

impl Simulation {
    pub fn random(
        rng: &mut dyn RngCore,
        generation_length: usize,
        animals: usize,
        foods: usize,
    ) -> Self {
        Self {
            world: world::World::random(rng, animals, foods),
            genetic_algo: ga::GeneticAlgorithm::new(
                ga::selection::RoulleteWheelSelection::new(),
                ga::crossover::UniformCrossover::new(),
                // chance and coefficient chosen with trial and error
                // higher values cause more chaos
                ga::mutation::GaussianMutation::new(0.01, 0.3)
            ),
            age: 0,
            generation_length,
        }
    }

    pub fn world(&self) -> &world::World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.handle_collision(rng);
        self.handle_decisions();
        self.process_movement();

        self.age += 1;

        if self.age > self.generation_length {
            self.evolve(rng);
        }
    }

    fn process_movement(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation() * na::Vector2::new(animal.speed(), 0.0);

            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
        }
    }

    fn handle_collision(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.food {
                let dist = na::distance(
                    &animal.position(),
                    &food.position()
                );

                if dist < 0.015 {
                    // nom-nom-nom
                    animal.food_consumed += 1;
                    food.position = rng.gen()
                }
            }
        }
    }

    fn handle_decisions(&mut self) {
        // for each animal
        for animal in &mut self.world.animals {
            // process vision
            let vision = animal.eye
                .process_vision(
                    animal.position,
                    animal.rotation,
                    &self.world.food
                );

            // get "decisions" from brain
            let decisions = animal.brain.neural_network
                .propagate(vision);

            // decision #1: speed change
            let delta_speed = decisions[0]
                .clamp(
                    -LIN_ACCELERATION,
                    LIN_ACCELERATION
                );

            // decision #2: change in direction
            let delta_theta = decisions[1]
                .clamp(
                    -ROT_ACCELERATION,
                    ROT_ACCELERATION
                );

            // Apply the changes made from "decisions"
            animal.speed = (animal.speed + delta_speed)
                .clamp(
                    MIN_SPEED,
                    MAX_SPEED
                );

            animal.rotation = na::Rotation2::new(
                animal.rotation.angle() + delta_theta
            );
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Prepare animals to be fed into the genetic algorithm
        let current_population: Vec<_> = self.world.animals
            .iter()
            .map(individual::AnimalIndividual::from_animal)
            .collect();

        // Evolve the animals
        let evolved_population = self.genetic_algo
            .iterate(rng, &current_population);

        // Prepare the evolved population for the simulation
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.as_animal(rng))
            .collect();

        // Prepare the food
        for food in &mut self.world.food {
            food.position = rng.gen();
        }
    }
}
