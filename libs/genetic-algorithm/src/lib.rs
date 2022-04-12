use std::ops::Index;

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

        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("population cannot be zero")
    }
}

pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn mut_iter(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter
                .into_iter()
                .collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn iterate<I>(
        &self,
        rng: &mut dyn RngCore,
        population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        (0..population.len())
            .map(|_| {
                // parent selection
                let parent_a = self
                    .selection_method
                    .select(rng, population);
                let parent_b = self
                    .selection_method
                    .select(rng, population);
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
    use std::collections::BTreeMap;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use maplit::btreemap;

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
