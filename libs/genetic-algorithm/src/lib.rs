use mutation::MutationMethod;
// #![feature(type_alias_impl_trait)]
use rand::RngCore;

mod individual;
mod selection;
mod crossover;
mod mutation;

pub struct GeneticAlgorithm<S, C> {
    selection_method: S,
    crossover_method: C,
    mutation_method: Box<dyn MutationMethod>
}

impl<S, C> GeneticAlgorithm<S, C>
where
    S: selection::SelectionMethod,
    C: crossover::CrossoverMethod
{
    pub fn new(
        selection_method: S,
        crossover_method: C,
        mutation_method: impl MutationMethod + 'static
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
