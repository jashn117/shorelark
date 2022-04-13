use rand::Rng;

pub struct Neuron {
    pub bias: f32, // Neuron's bias
    pub weights: Vec<f32>, // incoming synapse weights
}

impl Neuron {
    pub fn randomize(output_size: usize, rng: &mut dyn rand::RngCore) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    pub fn propagate(&self, inputs: &Vec<f32>) -> f32 {
        // number of inputs to neuron == number of synapses(weights)
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (self.bias + output).max(0.0)
    }
}

#[cfg(test)]
mod tests {
  use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::assert_relative_eq;

    mod neuron {
        use super::*;

        #[test]
        fn randomize_test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::randomize(4, &mut rng);

            assert_relative_eq!(neuron.bias, -0.6255188);

            assert_relative_eq!(neuron.weights[0], 0.67383957);
            assert_relative_eq!(neuron.weights[1], 0.8181262);
            assert_relative_eq!(neuron.weights[2], 0.26284897);
            assert_relative_eq!(neuron.weights[3], 0.5238807);
        }

        #[test]
        fn propagate_test() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![0.5, 0.1, 0.35],
            };

            assert_relative_eq!(
                neuron.propagate(&vec![-10.0, -15.0, -5.0]),
                0.0
            );

            assert_relative_eq!(
                neuron.propagate(&vec![0.79, 0.97, 0.13]),
                (0.5 * 0.79) + (0.1 * 0.97) + (0.35 * 0.13) + 0.5,
            );
        }
    }
}