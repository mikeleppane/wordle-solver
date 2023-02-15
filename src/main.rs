use clap::{Parser, ValueEnum};
use wordle_solver::algorithms::{Allocs, Naive};
use wordle_solver::{Guesser, Wordle};

const GAMES: &str = include_str!("../word-stat-generator/data/answers.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_enum)]
    implementation: Implementation,

    #[clap(short, long)]
    max: Option<usize>,
}

#[derive(ValueEnum, Debug, Clone)]
enum Implementation {
    Naive,
    Allocs,
}
fn main() {
    let args = Args::parse();
    match args.implementation {
        Implementation::Naive => play(Naive::new, args.max),
        Implementation::Allocs => play(Allocs::new, args.max),
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let w = Wordle::new();
    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();
        if let Some(score) = w.play(answer, guesser) {
            println!("Guessed '{answer}' in {score}");
        } else {
            eprintln!("failed to guess");
        }
    }
}
