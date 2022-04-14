use super::neuron;

pub struct Layer {
    pub(crate) neurons: Vec<neuron::Neuron>,
}

impl Layer {
    pub(crate) fn randomize(
        input_neurons: usize,
        output_neurons: usize,
        rng: &mut dyn rand::RngCore
    ) -> Self {
        let neurons = (0..output_neurons)
        .map(|_| neuron::Neuron::randomize(input_neurons, rng))
        .collect();
        
        Self { neurons }
    }
    
    pub(crate) fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
        .iter()
        .map(|neuron| neuron.propagate(&inputs))
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::assert_relative_eq;
    
    mod layer {
        use super::*;
        
        #[test]
        fn randomize_test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::randomize(3, 2, &mut rng);
            
            assert_eq!(layer.neurons.len(), 2);
            
            assert_relative_eq!(
                layer.neurons[0].bias,
                -0.6255188
            );
            assert_relative_eq!(layer.neurons[0].weights.as_slice(), [
            0.67383957,
            0.8181262,
            0.26284897
            ].as_ref());
            
            assert_relative_eq!(
                layer.neurons[1].bias,
                0.5238807
            );
            assert_relative_eq!(layer.neurons[1].weights.as_slice(), [
            -0.53516835,
            0.069369674,
            -0.7648182
            ].as_ref());
        }
        
        #[test]
        fn propagate_test() {
            let layer = Layer {
                neurons: vec![
                neuron::Neuron {
                    bias: 0.2,
                    weights: vec![-0.1, 0.3]
                },
                neuron::Neuron {
                    bias: -0.5,
                    weights: vec![-0.1, 0.75]
                },
                ]
            };
            
            let result = layer
            .propagate(vec![0.5, -0.4]);
            
            assert_relative_eq!(result.as_ref(),[
            (0.5 * (-0.1)) + (-0.4 * 0.3) + 0.2,
            0.0 
            ].as_ref());
        }
    }
}