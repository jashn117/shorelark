use lib_genetic_algorithm as ga;
use rand::RngCore;

use super::world;

pub struct AnimalIndividual {
  fitness: f32,
  chromosome: ga::individual::Chromosome
}

impl AnimalIndividual {
  fn from_animal(animal: &world::Animal) -> Self {
    Self {
      fitness: animal.food_consumed as f32,
      chromosome: todo!()
    }
  }

  fn as_animal(self, rng: &mut dyn RngCore) -> world::Animal {
    todo!()
  }
}

impl ga::individual::Individual for AnimalIndividual {
  fn from_chromosome(chromosome: ga::individual::Chromosome) -> Self {
    Self {
      fitness: 0.0,
      chromosome: chromosome
    }
  }

  fn as_chromosome(&self) -> &ga::individual::Chromosome {
    &self.chromosome
  }

  fn fitness(&self) -> f32 {
    self.fitness
  }
}
