use nalgebra as na;
use rand::{Rng, RngCore};

pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(), // na::Point2::new(rng.gen(), rng.gen())
            rotation: rng.gen(), // na::Rotation2::new(rng.gen())
            speed: 0.002
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }
}

pub struct Food {
    pub(crate) position: na::Point2<f32>
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen() // na::Point2::new(rng.gen(), rng.gen())
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}

pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) food: Vec<Food>
}

impl World {
    //TODO: prevent entities in world from overlapping
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..30)
            .map(|_| Animal::random(rng))
            .collect();
        
        let food = (0..50)
            .map(|_| Food::random(rng))
            .collect();

        Self { animals, food }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn food(&self) -> &[Food] {
        &self.food
    }
}
