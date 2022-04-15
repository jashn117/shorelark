use rand::{Rng, RngCore};
use nalgebra as na;

mod eye;
pub mod world;

pub struct Simulation {
    world: world::World,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: world::World::random(rng)
        }
    }

    pub fn world(&self) -> &world::World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_movement();
        self.handle_collision(rng)
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
                    food.position = rng.gen()
                }
            }
        }
    }
}
