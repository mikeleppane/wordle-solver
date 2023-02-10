use wordle_solver::algorithms::Naive;
use wordle_solver::play;

const GAMES: &str = include_str!("../word-stat-generator/data/answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();
        play(answer, guesser);
    }
}
