use rand::{Rng, RngCore};

use super::individual::*;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome
    ) -> Chromosome;
}

pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome
    ) -> Chromosome {
        assert_eq!(parent_a.len(), parent_b.len());
        
        //* Idomatic approach, doesn't work at the moment
        //TODO: Fix issues with trait IntoIterator impl for Chromosome
        // parent_a
        //     .iter()
        //     .zip(parent_b)
        //     .map(|(&a, b)| {
        //         if rng.gen_bool(0.5) {
        //             a 
        //         } else {
        //             b 
        //         }
        //     })
        //     .collect()

        let mut child = Vec::new();

        for idx in 0..parent_a.len() {
            let gene = if rng.gen_bool(0.5) {
                parent_a[idx]
            } else {
                parent_b[idx]
            };
            
            child.push(gene);
        }

        child
        .into_iter()
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    
    #[test]
    fn uniform_crossover_test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        // parents no gene in common
        let parent_a = Chromosome {
            genes: (1..=100)
            .map(|n| n as f32)
            .collect()
        };
        
        let parent_b = Chromosome {
            genes: (1..=100)
            .map(|n| -n as f32)
            .collect()
        };

        let child = UniformCrossover::new()
            .crossover(&mut rng, &parent_a, &parent_b);

        // "genes" different from parent_a
        let delta_a = child
            .iter()
            .zip(parent_a)
            .filter(|(c, p)| *c != p)
            .count();

        assert_eq!(delta_a, 49);
    }
}
