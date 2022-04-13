mod neuron;
mod layer;

//TODO: Implement a better interface for creating networks
pub struct LayerTopology {
    pub neurons: usize,
}

pub struct Network {
    layers: Vec<layer::Layer>,
}

impl Network {
    pub fn randomize(layers: &[LayerTopology], rng: &mut dyn rand::RngCore) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| {
                layer::Layer::randomize(layers[0].neurons, layers[1].neurons, rng)
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
                    // layer::Layer {
                    //     neurons: vec![
                    //         neuron::Neuron {
                    //             bias: 0.3,
                    //             weights: vec![-0.15]
                    //         },
                    //         neuron::Neuron {
                    //             bias: 0.5,
                    //             weights: vec![0.25]
                    //         },
                    //         neuron::Neuron {
                    //             bias: 0.1,
                    //             weights: vec![0.0]
                    //         }
                    //     ],
                    // },
                    layer::Layer {
                        neurons: vec![
                            neuron::Neuron {
                                bias: 0.62,
                                weights: vec![0.33, /*0.2, -0.4*/]
                            },
                            neuron::Neuron {
                                bias: 0.25,
                                weights: vec![-0.1, /*0.18, 0.45*/]
                            }
                        ],
                    },
                    layer::Layer {
                        neurons: vec![
                            neuron::Neuron {
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
