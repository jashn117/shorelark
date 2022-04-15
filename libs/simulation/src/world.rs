use nalgebra as na;
use rand::{Rng, RngCore};
use lib_neural_network as nn;

use super::eye;

pub struct Animal {
    pub(crate) brain: nn::Network,
    pub(crate) eye: eye::Eye,
    pub(crate) food_consumed: usize,
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = eye::Eye::default();

        let brain = nn::Network::randomize(
            &[
                // input layer
                nn::LayerTopology {
                    // one neuron each photoreceptor
                    neurons: eye.photoreceptors()
                },
                // hidden layer(s)
                nn::LayerTopology {
                    // Trial #1: twice that of the input layer
                    neurons: 2 * eye.photoreceptors()
                },
                // output layer
                nn::LayerTopology {
                    // two neurons: one for speed, other for rotation
                    neurons: 2
                }
            ],
            rng
        );

        Self {
            brain,
            eye,
            food_consumed: 0,
            position: rng.gen(), // na::Point2::new(rng.gen(), rng.gen())
            rotation: rng.gen(), // na::Rotation2::new(rng.gen())
            //TODO: slow down the simulation to a reasonable speed
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
