use rand::Rng;

struct Neuron {
    bias: f32, // Neuron's bias
    weights: Vec<f32>, // incoming synapse weights
}

impl Neuron {
    fn randomize(output_size: usize, rng: &mut dyn rand::RngCore) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &Vec<f32>) -> f32 {
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

struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn randomize(
        input_neurons: usize,
        output_neurons: usize,
        rng: &mut dyn rand::RngCore
    ) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::randomize(input_neurons, rng))
            .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

pub struct LayerTopology {
    pub neurons: usize,
}

pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn randomize(layers: &[LayerTopology], rng: &mut dyn rand::RngCore) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::randomize(layers[0].neurons, layers[1].neurons, rng)
            })
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
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
                    Neuron {
                        bias: 0.2,
                        weights: vec![-0.1, 0.3]
                    },
                    Neuron {
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

    mod network {
        use super::*;

        #[test]
        fn randomize_test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let network = Network::randomize(&[
                LayerTopology {
                    neurons: 1
                },
                LayerTopology {
                    neurons: 2
                },
                LayerTopology {
                    neurons: 1
                }
            ], &mut rng);

            assert_eq!(network.layers.len(), 2);
            assert_eq!(network.layers[0].neurons.len(), 2);
            assert_eq!(network.layers[1].neurons.len(), 1);

            assert_relative_eq!(
                network.layers[0].neurons[0].bias,
                -0.6255188
            );
            assert_relative_eq!(
                network.layers[0].neurons[0].weights.as_slice(),
                [0.67383957].as_ref()
            );

            assert_relative_eq!(
                network.layers[0].neurons[1].bias,
                0.8181262
            );
            assert_relative_eq!(
                network.layers[0].neurons[1].weights.as_slice(),
                [0.26284897].as_ref()
            );

            assert_relative_eq!(
                network.layers[1].neurons[0].bias,
                0.5238807
            );
            assert_relative_eq!(
                network.layers[1].neurons[0].weights.as_slice(),
                [-0.53516835, 0.069369674].as_ref()
            );
        }

        //TODO: Test fails, check for mistakes
        //? Test fails with three layers
        #[test]
        fn propagate_test() {
            let network = Network {
                layers: vec![
                    // Layer {
                    //     neurons: vec![
                    //         Neuron {
                    //             bias: 0.3,
                    //             weights: vec![-0.15]
                    //         },
                    //         Neuron {
                    //             bias: 0.5,
                    //             weights: vec![0.25]
                    //         },
                    //         Neuron {
                    //             bias: 0.1,
                    //             weights: vec![0.0]
                    //         }
                    //     ],
                    // },
                    Layer {
                        neurons: vec![
                            Neuron {
                                bias: 0.62,
                                weights: vec![0.33, /*0.2, -0.4*/]
                            },
                            Neuron {
                                bias: 0.25,
                                weights: vec![-0.1, /*0.18, 0.45*/]
                            }
                        ],
                    },
                    Layer {
                        neurons: vec![
                            Neuron {
                                bias: 0.17,
                                weights: vec![0.3, 0.5]
                            },
                        ],
                    }
                ],
            };

            let result = network
                .propagate(vec![0.37]);

            // let l1 = vec![
            //     (-0.15 * 0.37) + 0.3,
            //     (0.25 * 0.37) + 0.5,
            //     (0.0 * 0.37) + 0.1
            // ];
            let l2 = vec![
                (0.33 * 0.37) /*+ (0.2 * 0.37) + (-0.4 * 0.37)*/ + 0.62,
                (-0.1 * 0.37) /*+ (0.18 * 0.37) + (-0.45 * 0.37)*/ + 0.25
            ];
            let l3 = vec![
                (0.3 * l2[0]) + (0.5 * l2[1]) + 0.17
            ];

            assert_relative_eq!(result.as_slice(), l3.as_ref());
        }
    }
}
