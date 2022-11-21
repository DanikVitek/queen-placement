use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::chromosome::Chromosome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board<'ch> {
    pub(crate) chromosome: &'ch Chromosome,
}

impl<'ch> From<&'ch Chromosome> for Board<'ch> {
    fn from(chromosome: &'ch Chromosome) -> Self {
        Self { chromosome }
    }
}

impl<'ch> From<Board<'ch>> for &'ch Chromosome {
    fn from(b: Board<'ch>) -> Self {
        b.chromosome
    }
}

impl<'ch> Board<'ch> {
    pub fn has_beats(&self) -> bool {
        self.chromosome.genes.par_iter().enumerate().any(|(i, queen1)| {
            self.chromosome
                .genes
                .par_iter()
                .enumerate()
                .any(|(j, queen2)| i != j && (queen1 == queen2 || (i.abs_diff(j) == queen1.abs_diff(*queen2) as usize)))
        })
    }

    pub fn beats_count(&self) -> u16 {
        self.chromosome
            .genes
            .par_iter()
            .enumerate()
            .filter(|(i, q1)| {
                self.chromosome.genes.par_iter().enumerate().any(|(j, q2)| {
                    i != &j && (q1 == &q2 || (i.abs_diff(j) == q1.abs_diff(*q2) as usize))
                })
            })
            .count() as u16
    }

    /// Returns the fitness of this [`Board`].
    ///
    /// The goal is to maximize the function to be 1
    pub fn fitness(&self) -> f32 {
        1.0 / (self.beats_count() as f32 + 1.0)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::board::Board;
    use crate::chromosome::Chromosome;

    #[test]
    fn test_has_beats() {
        assert!(Board::from(Chromosome { genes: vec![0, 0] }).has_beats());
        assert!(Board::from(Chromosome { genes: vec![0, 1] }).has_beats());
        assert!(!Board::from(Chromosome { genes: vec![0, 2] }).has_beats());
        assert!(Board::from(Chromosome { genes: vec![1, 0] }).has_beats());
        assert!(!Board::from(Chromosome { genes: vec![2, 0] }).has_beats());
        assert!(Board::from(Chromosome {
            genes: vec![0, 2, 1]
        })
        .has_beats());
        assert!(Board::from(Chromosome {
            genes: vec![0, 2, 2]
        })
        .has_beats());
        assert!(!Board::from(Chromosome {
            genes: vec![0, 2, 4]
        })
        .has_beats());
    }

    #[test]
    fn test_beats_count() {
        assert_eq!(
            2,
            Board::from(Chromosome { genes: vec![0, 0] }).beats_count()
        );
        assert_eq!(
            2,
            Board::from(Chromosome { genes: vec![0, 1] }).beats_count()
        );
        assert_eq!(
            0,
            Board::from(Chromosome { genes: vec![0, 2] }).beats_count()
        );
        assert_eq!(
            2,
            Board::from(Chromosome { genes: vec![1, 0] }).beats_count()
        );
        assert_eq!(
            0,
            Board::from(Chromosome { genes: vec![2, 0] }).beats_count()
        );
        assert_eq!(
            2,
            Board::from(Chromosome {
                genes: vec![0, 2, 1]
            })
            .beats_count()
        );
        assert_eq!(
            3,
            Board::from(Chromosome {
                genes: vec![0, 2, 2]
            })
            .beats_count()
        );
        assert_eq!(
            0,
            Board::from(Chromosome {
                genes: vec![0, 2, 4]
            })
            .beats_count()
        );
    }
}
