use rand::seq::SliceRandom;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::selection::Generation;

pub type Gene = u16;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chromosome {
    pub(crate) genes: Vec<Gene>,
}

impl Chromosome {
    pub fn new(base: u16) -> Self {
        let mut genes: Vec<Gene> = (0..base).collect();
        genes.shuffle(&mut rand::thread_rng());
        Self { genes }
    }

    #[inline]
    pub fn genes(&self) -> &[Gene] {
        &self.genes
    }

    pub fn create_generation(board_size: u16, population_size: usize) -> Generation {
        (0..population_size)
            .into_par_iter()
            .map(|_| Self::new(board_size))
            .collect()
    }
}

impl From<Vec<Gene>> for Chromosome {
    #[inline]
    fn from(genes: Vec<Gene>) -> Self {
        Self { genes }
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
}
