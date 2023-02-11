use wordle_solver::algorithms::Naive;
use wordle_solver::Wordle;

const GAMES: &str = include_str!("../word-stat-generator/data/answers.txt");

fn main() {
    let w = Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();
        w.play(answer, guesser);
    }
}
