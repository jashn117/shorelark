use rand::RngCore;
use rand::seq::SliceRandom;

use super::individual::*;

pub trait SelectionMethod {
    fn select<'a, I>(
        &self,
        rng: &mut dyn RngCore,
        population: &'a [I]
    ) -> &'a I
    where
        I: Individual;
}

pub struct RoulleteWheelSelection;

impl RoulleteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RoulleteWheelSelection {
    fn select<'a, I>(
        &self,
        rng: &mut dyn RngCore,
        population: &'a [I]
    ) -> &'a I
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("population cannot be zero")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use maplit::btreemap;

    // * Testing structs
    #[cfg(test)]
    #[derive(Clone, Debug)]
    pub struct TestIndividual {
        fitness: f32,
    }

    #[cfg(test)]
    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self { fitness }
        }
    }

    #[cfg(test)]
    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.fitness
        }
        fn chromosome(&self) -> &Chromosome {
            panic!("Not implemented for TestIndividual")
        }
    }

    mod select {
        use super::*;

        #[test]
        fn roullete_wheel() {
            let method = RoulleteWheelSelection::new();
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let population = vec![
                TestIndividual::new(1.0),
                TestIndividual::new(2.0),
                TestIndividual::new(3.0),
                TestIndividual::new(5.0)
            ];

            let actual_histogram = (0..1000)
                .map(|_| method.select(&mut rng, &population))
                .fold(BTreeMap::default(), |mut histogram, individual| {
                    *histogram
                        .entry(individual.fitness() as i32)
                        .or_default() += 1;

                    histogram
                });

            let expected_histogram = btreemap!{
                // fitness => times selected by selection method
                1 => 96,
                2 => 174,
                3 => 285,
                5 => 445
            };

            assert_eq!(actual_histogram, expected_histogram);
        }
    }
}
