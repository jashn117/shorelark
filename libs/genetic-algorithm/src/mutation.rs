use rand::{Rng,RngCore};

use super::individual::*;

pub trait MutationMethod {
    fn mutate(
        &self,
        rng: &mut dyn RngCore,
        child: &mut Chromosome
    );
}

pub struct GaussianMutation {
    //* probability of changing n genes
    //* if n = 0.0, no gene is slated for mutation
    //* if n = 1.0, all genes are slated for mutation
    chance: f32,
    //* Magnitude of the change
    //* if n = 0.0, no mutation
    //* if n = 1.0, mutation upto +-3.0
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && coeff >= 0.0);

        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.mut_iter() {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

            if rng.gen_bool(self.chance as _) {
                *gene += sign * self.coeff * rng.gen::<f32>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::*;

    fn helper(chance: f32, coeff: f32) -> Vec<f32> {
        let mut child = Chromosome {
            genes: vec![1.0, -3.4, 2.5, 3.2]
        };

        let mut rng = ChaCha8Rng::from_seed(Default::default());

        GaussianMutation::new(chance, coeff)
            .mutate(&mut rng, &mut child);

        child
            .into_iter()
            .collect()
    }

    mod given_zero_chance {
        use super::*;

        fn actual(coeff: f32) -> Vec<f32> {
            super::helper(0.0, coeff)
        }

        mod and_zero_coeff {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, -3.4, 2.5, 3.2];

                assert_relative_eq!(
                    actual.as_slice(),
                    expected.as_slice()
                )
            }
        }

        mod and_nonzero_coeff {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.75);
                let expected = vec![1.0, -3.4, 2.5, 3.2];

                assert_relative_eq!(
                    actual.as_slice(),
                    expected.as_slice()
                )
            }
        }
    }

    mod given_fifty_fifty_chance {
        use super::*;

        fn actual(coeff: f32) -> Vec<f32> {
            super::helper(0.5, coeff)
        }

        mod and_zero_coeff {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, -3.4, 2.5, 3.2];

                assert_relative_eq!(
                    actual.as_slice(),
                    expected.as_slice()
                )
            }
        }

        mod and_nonzero_coeff {
            use super::*;

            #[test]
            fn changes_the_original_chromosome() {
                let actual = actual(0.4);
                let expected = vec![1.0, -3.4, 2.5, 3.2];

                assert_relative_ne!(
                    actual.as_slice(),
                    expected.as_slice()
                )
            }
        }
    }

    mod given_max_chance {
        use super::*;

        fn actual(coeff: f32) -> Vec<f32> {
            super::helper(1.0, coeff)
        }

        mod and_zero_coeff {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, -3.4, 2.5, 3.2];

                assert_relative_eq!(
                    actual.as_slice(),
                    expected.as_slice()
                )
            }
        }

        mod and_nonzero_coeff {
            use super::*;

            #[test]
            fn entirely_changes_the_original_chromosome() {
                let actual = actual(0.6);
                let expected = vec![1.0, -3.4, 2.5, 3.2];

                assert_relative_ne!(
                    actual.as_slice(),
                    expected.as_slice()
                )
            }
        }
    }
}
