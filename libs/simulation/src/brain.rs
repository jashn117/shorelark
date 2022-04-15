use lib_neural_network as nn;
use lib_genetic_algorithm as ga;
use rand::RngCore;

use super::eye;

pub struct Brain {
    pub(crate) neural_network: nn::Network,
}

impl Brain {
    pub fn randomize(rng: &mut dyn RngCore, eye: &eye::Eye) -> Self {
        Self {
            neural_network: nn::Network::randomize(&Self::topology(eye), rng)
        }
    }
    
    pub fn as_chromosome(&self) -> ga::individual::Chromosome {
        ga::individual::Chromosome {
            genes: self.neural_network
                .weights()
        }
    }
    
    pub fn topology(eye: &eye::Eye) -> [nn::LayerTopology; 3] {
        [
        // input layer
        // neuron for each photoreceptor
        nn::LayerTopology {
            neurons: eye.photoreceptors(),
        },
        // hidden layer(s)
        // Trial #1: 2x input layer's neurons
        nn::LayerTopology {
            neurons: 2 * eye.photoreceptors(),
        },
        // output layer
        // two neurons, one for speed, other for rotation/direction
        nn::LayerTopology {
            neurons: 2,
        }
        ]
    }
}
