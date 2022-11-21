use rand::seq::SliceRandom;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::chromosome::Chromosome;

pub type Generation = Vec<Chromosome>;

pub fn new_generation(current_generation: Generation) -> Generation {
    let population_size = current_generation.len();
    let new_generation = Generation::with_capacity(population_size);
    (0..population_size).into_par_iter().for_each(|_| {
        let parent1: &Chromosome = current_generation.choose_weighted(&mut rand::thread_rng(), |ch| {
            
        });
    });
    todo!();
}