use rand::RngCore;
use nalgebra as na;

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

    pub fn step(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation() * na::Vector2::new(animal.speed(), 0.0);

            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
