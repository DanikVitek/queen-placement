use std::{
    collections::HashSet,
    io::{stdout, Write},
};

use clap::Parser;
use crossterm::{
    cursor, queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use queen_placement::{
    board::Board,
    chromosome::Chromosome,
    config::Config,
    selection::{self, Generation},
};
use rayon::prelude::{
    FromParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

fn main() {
    let Config {
        board_size,
        generation_size,
        mutation_probability,
        selection_strategy,
    } = Config::parse();

    if generation_size.get() < 2 {
        println!("Generation size os too small");
        return;
    }

    let mut generation = Chromosome::create_generation(board_size.get(), generation_size.get());
    let mut generation_count: u32 = 1;
    print_max_fitness(&generation, &generation_count);

    while !generation.par_iter().any(is_fitting) {
        generation =
            selection::new_generation(selection_strategy, generation, mutation_probability);
        generation_count += 1;
        print_max_fitness(&generation, &generation_count);
    }

    let mut stdout = stdout();
    queue!(
        stdout,
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        crossterm::cursor::MoveTo(0, 0)
    )
    .unwrap();
    stdout.flush().unwrap();
    let fitting_individuals: HashSet<Chromosome> =
        HashSet::from_par_iter(generation.into_par_iter().filter(is_fitting));
    for individual in fitting_individuals {
        let board = Board::from(&individual);
        println!("{board}\n({generation_count})");
    }
}

fn is_fitting(ch: &Chromosome) -> bool {
    Board::from(ch).fitness() == 1.0
}

fn print_max_fitness(gen: &Generation, generation_count: &u32) {
    let (b, ch) = gen
        .par_iter()
        .map(|chromosome| {
            let board = Board::from(chromosome);
            let fitness = board.fitness();
            (board, fitness)
        })
        .max_by(|(_, f1), (_, f2)| f32::partial_cmp(f1, f2).unwrap())
        .unwrap();

    let mut stdout = stdout();
    queue!(
        stdout,
        Clear(ClearType::All),
        Clear(ClearType::Purge),
        cursor::MoveTo(0, 0),
        Print(format!("{ch} ({generation_count})\n{b}"))
    )
    .unwrap();
    stdout.flush().unwrap();
}
