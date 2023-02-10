const GAMES: &str = include_str!("../word-stat-generator/data/answers.txt");

fn main() {
    println!("Hello, world!");
}

fn play<G: Guesser>(answer: &'static str, guesser: G) {
    todo!();
}

enum Correctness {
    Correct,
    Present,
    Misplaced,
}

struct Guess {
    word: String,
    mask: [Correctness; 5],
}

trait Guesser {
    fn guess(&mut self, past: &[Guess]) -> String;
}
