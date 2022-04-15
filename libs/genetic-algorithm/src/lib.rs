// #![feature(type_alias_impl_trait)]
use rand::RngCore;

pub mod individual;
pub mod selection;
pub mod crossover;
pub mod mutation;

pub struct GeneticAlgorithm<S, C> {
    selection_method: S,
    crossover_method: C,
    mutation_method: Box<dyn mutation::MutationMethod>
}

impl<S, C> GeneticAlgorithm<S, C>
where
    S: selection::SelectionMethod,
    C: crossover::CrossoverMethod
{
    pub fn new(
        selection_method: S,
        crossover_method: C,
        mutation_method: impl mutation::MutationMethod + 'static
    ) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method: Box::new(mutation_method)
        }
    }

    pub fn iterate<I>(
        &self,
        rng: &mut dyn RngCore,
        population: &[I]) -> Vec<I>
    where
        I: individual::Individual,
    {
        (0..population.len())
            .map(|_| {
                // Step #1: parent selection
                let parent_a = self
                    .selection_method
                    .select(rng, population)
                    .as_chromosome();
                let parent_b = self
                    .selection_method
                    .select(rng, population)
                    .as_chromosome();

                // Step #2: crossover/mix "traits"
                let mut child = self
                    .crossover_method
                    .crossover(rng, parent_a, parent_b);

                // Step #3: mutation
                self
                    .mutation_method
                    .mutate(rng, &mut child);

                I::from_chromosome(child)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::individual::Individual;

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::{relative_eq};

    // * Testing structs
    #[cfg(test)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct TestIndividual {
        fitness: f32,
        chromosome: individual::Chromosome,
    }

    impl PartialEq for individual::Chromosome {
        fn eq(&self, other: &Self) -> bool {
            relative_eq!(
                self.genes.as_slice(),
                other.genes.as_slice()
            )
        }
    }

    #[cfg(test)]
    impl TestIndividual {
        pub fn new(fitness: f32, chromosome: individual::Chromosome) -> Self {
            Self { fitness, chromosome }
        }
    }

    #[cfg(test)]
    impl individual::Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            self.chromosome
                .iter()
                .sum()
        }

        fn as_chromosome(&self) -> &individual::Chromosome {
            &self.chromosome
        }

        fn from_chromosome(chromosome: individual::Chromosome) -> Self {
            let fitness = chromosome
                .iter()
                .sum();

            Self::new(fitness, chromosome)
        }
    }

    fn create_individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes
            .iter()
            .cloned()
            .collect();

        TestIndividual::from_chromosome(chromosome)
    }

    mod select {
        use super::*;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());

            let genetic_algo = GeneticAlgorithm::new(
                selection::RoulleteWheelSelection::new(),
                crossover::UniformCrossover::new(),
                mutation::GaussianMutation::new(0.5, 0.5)
            );

            let mut population = vec![
                create_individual(&[0.0, 0.0, 0.0, 0.0]),
                create_individual(&[0.7, 2.1, -0.5, 3.3]),
                create_individual(&[1.3, 1.7, 2.4, 0.0]),
                create_individual(&[-0.1, 0.0, 2.1, 1.1]),
                create_individual(&[0.3, 0.9, 2.0, 2.8]),
            ];

            for _ in 0..10 {
                population = genetic_algo
                    .iterate(&mut rng, &population);
            }

            let expected_population = vec![
                create_individual(&[1.2499224, 1.9505982, -1.3171668, 4.565424]),
                create_individual(&[1.2497408, 1.3241994, -1.6529529, 3.3563695]),
                create_individual(&[2.2138042, 1.6062636, -1.3674062, 3.8496184]),
                create_individual(&[1.0064117, 1.9913193, -1.3171668, 4.0800223]),
                create_individual(&[2.359843, 2.371323, -0.79184055, 4.5782356]),
            ];

            assert_eq!(
                population,
                expected_population
            );
        }
    }
}
