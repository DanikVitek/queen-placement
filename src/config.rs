use std::fmt;
use std::num::NonZeroUsize;
use std::str::FromStr;

use clap::{builder::PossibleValue, Parser, ValueEnum};

use crate::selection::{Probability, SelectionStrategy};

#[derive(Parser, Debug)]
#[command(author, about)]
pub struct Config {
    /// Size of the chess board
    #[arg(short, long, default_value_t = 8)]
    pub board_size: u16,

    /// Size of the population in one generation
    #[arg(short, long, default_value_t = NonZeroUsize::try_from(100).unwrap())]
    pub generation_size: NonZeroUsize,

    /// Probability of mutation
    #[arg(short = 'p', long, default_value_t = Probability(0.1))]
    pub mutation_probability: Probability,

    /// Strategy for selecting the best individuals for the next generation
    #[arg(short, long, default_value_t = SelectionStrategy::AdamAndEve)]
    pub selection_strategy: SelectionStrategy,
}

impl fmt::Display for Probability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Probability {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.parse()?)
    }
}

impl fmt::Display for SelectionStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectionStrategy::AdamAndEve => write!(f, "Adam and Eve"),
            SelectionStrategy::KillTheHalf => write!(f, "Kill the half"),
            SelectionStrategy::Tournament => write!(f, "Tournament"),
        }
    }
}

impl ValueEnum for SelectionStrategy {
    fn value_variants<'a>() -> &'a [Self] {
        use SelectionStrategy::*;
        &[AdamAndEve, KillTheHalf, Tournament]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.to_string()))
    }
}
