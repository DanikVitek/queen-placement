use rand::{distributions::Uniform, prelude::Distribution, seq::SliceRandom, Rng};
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use std::sync::{Arc, Mutex, MutexGuard};

pub type Gene = u16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chromosome {
    pub(crate) genes: Vec<Gene>,
}

impl Chromosome {
    pub fn new(base: u16) -> Self {
        let mut genes = (0..base).collect::<Vec<Gene>>();
        genes.shuffle(&mut rand::thread_rng());
        Self { genes }
    }

    pub fn genes(&self) -> &[Gene] {
        &self.genes
    }

    /// With the given probability swaps two random genes.
    pub fn maybe_mutate(&mut self, probability: f32) {
        assert!((0.0..=1.0).contains(&probability));
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < probability {
            let sampler = Uniform::from(0..self.genes.len());
            let idx1 = sampler.sample(&mut rng);
            let mut idx2 = sampler.sample(&mut rng);
            while idx2 == idx1 {
                idx2 = sampler.sample(&mut rng);
            }
            self.genes.swap(idx1, idx2);
        }
    }

    /// Creates new population from parent chromosomes: `self` and `other`.
    /// New population will consist of the chromosomes with genes that are
    /// equal to the parents' where their genes are equal and distinct random
    /// ones in other places.
    pub fn new_generation(self, other: Self, population_size: usize) -> Vec<Self> {
        if population_size == 0 {
            return Vec::new();
        }

        let similar_genes: Vec<Option<Gene>> = self
            .genes
            .par_iter()
            .zip(other.genes.par_iter())
            .map(|(s, o)| (s == o).then_some(s.to_owned()))
            .collect();
        let gene_digit_base = similar_genes.len() as u16;
        let non_similar_genes_indices: Vec<usize> = similar_genes
            .par_iter()
            .enumerate()
            .filter_map(|(i, v)| v.is_none().then_some(i))
            .collect();

        let new_gen: Arc<Mutex<Vec<Vec<Option<Gene>>>>> = Arc::new(Mutex::new(Vec::<Vec<Option<Gene>>>::with_capacity(
            population_size + 2,
        )));

        (0..population_size).into_par_iter().for_each(|_| {
            let new_gen: Arc<Mutex<Vec<Vec<Option<u16>>>>> = Arc::clone(&new_gen);
            let mut new_gen_guard = new_gen.lock().unwrap();
            new_gen_guard.push(similar_genes.to_owned());

            Self::replace_nones(new_gen_guard, gene_digit_base, &non_similar_genes_indices);
        });

        // unwrapping chromosomes from smart pointers
        let mut new_gen: Vec<Chromosome> = Arc::try_unwrap(new_gen)
            .unwrap()
            .into_inner()
            .unwrap()
            .into_par_iter()
            .map(|chromosome| Self {
                genes: chromosome
                    .into_par_iter()
                    .map(|gene| gene.unwrap())
                    .collect(),
            })
            .collect();

        new_gen.push(self);
        new_gen.push(other);
        new_gen
    }

    fn replace_nones(
        mut guard: MutexGuard<Vec<Vec<Option<u16>>>>,
        base: u16,
        non_similar_genes_indices: &[usize],
    ) {
        let chromosome = guard.last_mut().unwrap();
        let mut rng = rand::thread_rng();
        let sampler = Uniform::from(0..base);
        non_similar_genes_indices.iter().for_each(|i| {
            let mut gene: Gene = sampler.sample(&mut rng);
            while chromosome.par_iter().any(|v| match v {
                Some(v) => v == &gene,
                None => false,
            }) {
                gene = sampler.sample(&mut rng);
            }
            chromosome[*i] = Some(gene);
        });
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_gene_new() {
        let len = 5;
        let chromosome = Chromosome::new(len);
        println!("{chromosome:#?}");
        assert_eq!(chromosome.genes.len(), len as usize);
    }

    #[test]
    fn test_new_generation() {
        let ch1 = Chromosome {
            genes: vec![0, 1, 4, 2, 3],
        };
        let ch2 = Chromosome {
            genes: vec![0, 1, 3, 4, 2],
        };
        let generation = ch1.new_generation(ch2, 3);
        assert_eq!(generation.len(), 3 + 2);
        for individual in generation {
            assert_eq!(individual.genes.len(), 5);
            assert!(individual.genes.iter().all(|gene| &0 <= gene && gene < &5));
            assert_eq!(individual.genes[0], 0);
            assert_eq!(individual.genes[1], 1);
            println!("{individual:#?}")
        }
    }
}
