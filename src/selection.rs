use std::collections::VecDeque;

use anyhow::ensure;
use rand::{distributions, prelude::Distribution, random, seq::SliceRandom};
use rayon::{
    prelude::{
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator,
        IntoParallelRefMutIterator, ParallelIterator,
    },
    slice::ParallelSliceMut,
};

use crate::{
    board::Board,
    chromosome::{Chromosome, Gene},
};

pub type Generation = Vec<Chromosome>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SelectionStrategy {
    AdamAndEve,
    KillTheHalf,
    Tournament,
}

pub fn new_generation(
    selection_strategy: SelectionStrategy,
    mut current_generation: Generation,
    mutation_probability: Probability,
) -> Generation {
    let population_size = current_generation.len();

    match selection_strategy {
        SelectionStrategy::AdamAndEve => adam_and_eve_impl(
            &mut current_generation,
            mutation_probability,
            population_size,
        ),
        SelectionStrategy::KillTheHalf => kill_the_half_impl(
            &mut current_generation,
            mutation_probability,
            population_size,
        ),
        SelectionStrategy::Tournament => tournament_impl(
            &mut current_generation,
            mutation_probability,
            population_size,
        ),
    }

    current_generation
}

fn adam_and_eve_impl(
    current_generation: &mut Generation,
    mutation_probability: Probability,
    population_size: usize,
) {
    let parent1 = take_max(current_generation);
    let parent2 = take_max(current_generation);

    (0..population_size - 2)
        .into_par_iter()
        .map(|_| crossover(&parent1, &parent2, mutation_probability))
        .collect_into_vec(current_generation); // reuse of already allocated memory

    current_generation.push(parent1);
    current_generation.push(parent2);
}

fn take_max(current_generation: &mut Vec<Chromosome>) -> Chromosome {
    current_generation.swap_remove(
        current_generation
            .par_iter()
            .enumerate()
            .max_by(|(_, ch1), (_, ch2)| {
                f32::partial_cmp(&Board::from(*ch1).fitness(), &Board::from(*ch2).fitness())
                    .unwrap()
            })
            .unwrap()
            .0,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
struct OrdF32(f32);

impl Eq for OrdF32 {}

impl Ord for OrdF32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

fn kill_the_half_impl(
    current_generation: &mut Generation,
    mutation_probability: Probability,
    population_size: usize,
) {
    current_generation.par_sort_by_cached_key(|ch| OrdF32(-Board::from(ch).fitness()));

    // p p 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 (22)
    let parent1 = current_generation.swap_remove(0);
    let parent2 = current_generation.swap_remove(1);

    // 0 0 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 (20)
    current_generation.truncate(population_size / 2);
    // 0 0 1 1 1 1 1 1 1 1 1
    current_generation.swap_remove(0);
    // 1 0 1 1 1 1 1 1 1 1
    current_generation.swap_remove(1);
    // 1 1 1 1 1 1 1 1 1

    current_generation.append(
        &mut (0..population_size - population_size / 2)
            .into_par_iter()
            .map(|_| crossover(&parent1, &parent2, mutation_probability))
            .collect::<Vec<_>>(),
    );

    current_generation.push(parent1);
    current_generation.push(parent2);
}

fn tournament_impl(
    _current_generation: &mut Generation,
    _mutation_probability: Probability,
    _population_size: usize,
) {
    todo!()
}

fn crossover(
    parent1: &Chromosome,
    parent2: &Chromosome,
    mutation_probability: Probability,
) -> Chromosome {
    let gene_digit_base = parent1.genes.len();
    #[cfg(debug_assertions)]
    {
        assert_eq!(gene_digit_base, parent2.genes.len());
    }

    // Mutation (in this case it is a new completely random [`Chromosome`])
    if random::<Probability>() < mutation_probability {
        return Chromosome::new(gene_digit_base as u16);
    }

    let mut similar_genes: Vec<Option<Gene>> = Vec::with_capacity(gene_digit_base);
    let gene_digit_base = gene_digit_base as u16;

    parent1
        .genes()
        .par_iter()
        .zip(parent2.genes())
        .map(|(p1, p2)| (p1 == p2).then_some(*p1))
        .collect_into_vec(&mut similar_genes);

    let rest_of_genes: Vec<Gene> = {
        let mut rest_of_genes: Vec<_> = (0..gene_digit_base)
            .into_par_iter()
            .filter(|g| !similar_genes.contains(&Some(*g)))
            .collect();
        rest_of_genes.shuffle(&mut rand::thread_rng());
        rest_of_genes
    };

    similar_genes
        .par_iter_mut()
        .filter(|g| g.is_none())
        .collect::<VecDeque<_>>()
        .into_par_iter()
        .zip(rest_of_genes.into_par_iter())
        .for_each(|(none, gene)| *none = Some(gene));

    similar_genes
        .into_par_iter()
        .map(Option::unwrap)
        .collect::<Vec<Gene>>()
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Probability(pub(crate) f32);

impl Probability {
    pub fn new(p: f32) -> anyhow::Result<Self> {
        ensure!(
            (0.0..=1.0).contains(&p),
            "Probability must be within the range [0.0; 1.0], but was {}",
            p
        );
        Ok(Self(p))
    }
}

impl TryFrom<f32> for Probability {
    type Error = anyhow::Error;

    #[inline]
    fn try_from(value: f32) -> anyhow::Result<Self> {
        Self::new(value)
    }
}

impl Distribution<Probability> for distributions::Standard {
    #[inline]
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Probability {
        Probability(rng.gen())
    }
}

#[cfg(test)]
mod tests {
    use rand::random;

    use crate::chromosome::Chromosome;

    use super::crossover;

    #[test]
    fn test_crossover() {
        let parent1 = Chromosome::new(5);
        let parent2 = Chromosome::new(5);
        let crossover = crossover(&parent1, &parent2, random());
        println!("{parent1:#?}\n{parent2:#?}\n{crossover:#?}");
        assert_eq!(5, crossover.genes.len());
    }
}
