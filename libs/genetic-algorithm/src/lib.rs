use rand::RngCore;
use rand::seq::SliceRandom;

pub trait Individual {
    fn fitness(&self) -> f32;
}

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

        // let total_fitness= population
        //     .iter()
        //     .map(|individual| individual.fitness())
        //     .sum::<f32>();

        // loop {
        //     let individual = population
        //         .choose(rng)
        //         .expect("got an empty population");

        //     let individual_share = individual
        //         .fitness() / total_fitness;

        //     if rng.gen_bool(individual_share as f64) {
        //         return individual;
        //     }
        // }
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("population cannot be zero")
    }
}

pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn iterate<I>(&self, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        (0..population.len())
            .map(|_| {
                //TODO: parent selection
                //TODO: crossover/mix "traits"
                //TODO: mutation
                todo!()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    //? Testing structs
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
    }

    mod select {
        use std::collections::BTreeMap;

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

            let mut actual_histogram = BTreeMap::new();

            // Generate the actual histogram
            for _ in  0..1000 {
                // Select fitness using the Roullete Wheel Selection Method
                let fitness = method
                    .select(&mut rng, &population)
                    .fitness() as i32;

                // Increment the number of times this fitness
                // was selected by selection method
                *actual_histogram
                    .entry(fitness)
                    .or_insert(0) += 1;
            }

            let expected_histogram = BTreeMap::from_iter(vec![
                (1, 96),
                (2, 174),
                (3, 285),
                (5, 445)
            ]);

            assert_eq!(actual_histogram, expected_histogram);
        }
    }
}
