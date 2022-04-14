use rand::RngCore;

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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
