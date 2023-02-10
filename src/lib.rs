use std::collections::{HashMap, HashSet};

pub mod algorithms;

pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    let mut history = Vec::new();
    for i in 1..32 {
        let guess = guesser.guess(&history);
        if guess == answer {
            return Some(i);
        }
        let correctness = Correctness::compute(answer, guess.as_str());
        history.push(Guess {
            word: guess,
            mask: correctness,
        });
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Wrong; 5];
        let mut visited = HashSet::<usize>::new();
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
                visited.insert(i);
                continue;
            }
            if answer.contains(g) {
                for (n, a) in answer.chars().enumerate() {
                    if visited.contains(&n) {
                        continue;
                    }
                    if a == g {
                        c[i] = Correctness::Misplaced;
                        visited.insert(n);
                        break;
                    }
                }
            }
        }
        c
    }
}

pub struct Guess {
    word: String,
    mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, past: &[Guess]) -> String;
}

#[cfg(test)]
mod tests {
    mod compute {
        use crate::Correctness;

        #[test]
        fn basic() {
            assert_eq!(
                Correctness::compute("abcde", "abcde"),
                [Correctness::Correct; 5]
            );
        }
        #[test]
        fn all_wrong() {
            assert_eq!(
                Correctness::compute("abcde", "hjikf"),
                [Correctness::Wrong; 5]
            );
        }
        #[test]
        fn all_misplaced() {
            assert_eq!(
                Correctness::compute("abcde", "edabc"),
                [Correctness::Misplaced; 5]
            );
        }

        #[test]
        fn correct_and_wrong() {
            assert_eq!(
                Correctness::compute("abcde", "ablke"),
                [
                    Correctness::Correct,
                    Correctness::Correct,
                    Correctness::Wrong,
                    Correctness::Wrong,
                    Correctness::Correct
                ]
            );
            assert_eq!(
                Correctness::compute("cccbb", "gggbb"),
                [
                    Correctness::Wrong,
                    Correctness::Wrong,
                    Correctness::Wrong,
                    Correctness::Correct,
                    Correctness::Correct
                ]
            );
        }
        #[test]
        fn correct_and_misplaced() {
            assert_eq!(
                Correctness::compute("abcde", "abdce"),
                [
                    Correctness::Correct,
                    Correctness::Correct,
                    Correctness::Misplaced,
                    Correctness::Misplaced,
                    Correctness::Correct
                ]
            );
            assert_eq!(
                Correctness::compute("abcde", "bacde"),
                [
                    Correctness::Misplaced,
                    Correctness::Misplaced,
                    Correctness::Correct,
                    Correctness::Correct,
                    Correctness::Correct
                ]
            );
        }
        #[test]
        fn correct_and_misplaced_and_wrong() {
            assert_eq!(
                Correctness::compute("abcde", "abdck"),
                [
                    Correctness::Correct,
                    Correctness::Correct,
                    Correctness::Misplaced,
                    Correctness::Misplaced,
                    Correctness::Wrong
                ]
            );
            assert_eq!(
                Correctness::compute("aabbc", "aaccb"),
                [
                    Correctness::Correct,
                    Correctness::Correct,
                    Correctness::Misplaced,
                    Correctness::Wrong,
                    Correctness::Misplaced
                ]
            );
        }
        #[test]
        fn misplaced_and_wrong() {
            assert_eq!(
                Correctness::compute("aabbb", "ccaac"),
                [
                    Correctness::Wrong,
                    Correctness::Wrong,
                    Correctness::Misplaced,
                    Correctness::Misplaced,
                    Correctness::Wrong
                ]
            );
            assert_eq!(
                Correctness::compute("bbbaa", "aaccc"),
                [
                    Correctness::Misplaced,
                    Correctness::Misplaced,
                    Correctness::Wrong,
                    Correctness::Wrong,
                    Correctness::Wrong
                ]
            );
        }
    }
}
