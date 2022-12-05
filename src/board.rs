use std::fmt;

use crossterm::style::{Color, Stylize};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::chromosome::Chromosome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board<'ch>(&'ch Chromosome);

impl<'ch> PartialOrd for Board<'ch> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness().partial_cmp(&other.fitness())
    }
}

impl<'ch> Ord for Board<'ch> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness().total_cmp(&other.fitness())
    }
}

impl<'ch> From<&'ch Chromosome> for Board<'ch> {
    #[inline]
    fn from(chromosome: &'ch Chromosome) -> Self {
        Self(chromosome)
    }
}

impl<'ch> From<Board<'ch>> for &'ch Chromosome {
    #[inline]
    fn from(b: Board<'ch>) -> Self {
        b.0
    }
}

impl<'ch> Board<'ch> {
    /// Amount of chess pieces that have been beaten
    pub fn beats_count(&self) -> u16 {
        self.0
            .genes
            .par_iter()
            .enumerate()
            .filter(|(x1, y1)| {
                self.0.genes.par_iter().enumerate().any(|(x2, y2)| {
                    x1 != &x2 && x1.abs_diff(x2) == y1.abs_diff(*y2) as usize // simplified version without horizontal intersection testing
                })
            })
            .count() as u16
    }

    /// Returns the fitness of this [`Board`].
    ///
    /// The goal is to maximize the function to be 1
    #[inline]
    pub fn fitness(&self) -> f32 {
        1.0 / (self.beats_count() as f32 + 1.0)
    }
}

impl<'ch> fmt::Display for Board<'ch> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.0.genes.len();

        let mut color = false; // black or white
        for y in 0..size as u16 {
            for x in 0..size {
                write!(
                    f,
                    "{}",
                    if self.0.genes[x] == y {
                        "##".dark_red()
                            .on(if color { Color::DarkGrey } else { Color::White })
                    } else {
                        "  ".on(if color { Color::DarkGrey } else { Color::White })
                    }
                )?;
                color = !color;
            }
            writeln!(f)?;
            if size % 2 == 0 {
                color = !color;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::board::Board;
    use crate::chromosome::Chromosome;

    #[test]
    fn test_beats_count() {
        assert_eq!(
            2,
            Board::from(&Chromosome { genes: vec![0, 0] }).beats_count()
        );
        assert_eq!(
            2,
            Board::from(&Chromosome { genes: vec![0, 1] }).beats_count()
        );
        assert_eq!(
            0,
            Board::from(&Chromosome { genes: vec![0, 2] }).beats_count()
        );
        assert_eq!(
            2,
            Board::from(&Chromosome { genes: vec![1, 0] }).beats_count()
        );
        assert_eq!(
            0,
            Board::from(&Chromosome { genes: vec![2, 0] }).beats_count()
        );
        assert_eq!(
            2,
            Board::from(&Chromosome {
                genes: vec![0, 2, 1]
            })
            .beats_count()
        );
        assert_eq!(
            3,
            Board::from(&Chromosome {
                genes: vec![0, 2, 2]
            })
            .beats_count()
        );
        assert_eq!(
            0,
            Board::from(&Chromosome {
                genes: vec![0, 2, 4]
            })
            .beats_count()
        );
    }
}
