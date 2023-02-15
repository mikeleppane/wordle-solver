use itertools::iproduct;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

pub mod algorithms;

const DICTIONARY: &str = include_str!("../word-stat-generator/data/dictionary-with-counts.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(" ")
                    .expect("every line is word + space + frequency")
                    .0
            })),
        }
    }
    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();
        for i in 1..32 {
            let guess = guesser.guess(&history);
            if guess == answer {
                return Some(i);
            }
            assert!(
                self.dictionary.contains(&*guess),
                "guess '{}' is not in the dictionary",
                guess
            );
            let correctness = Correctness::compute(answer, guess.as_str());
            history.push(Guess {
                word: Cow::Owned(guess),
                mask: correctness,
            });
        }
        None
    }
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
        let mut used = [false; 5];
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
                used[i] = true;
            }
        }
        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                continue;
            }
            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }
        c
    }

    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

pub struct Guess<'a> {
    word: Cow<'a, str>,
    mask: [Correctness; 5],
}

impl Guess<'_> {
    pub fn matches(&self, word: &str) -> bool {
        Correctness::compute(word, &self.word) == self.mask
    }
}

pub trait Guesser {
    fn guess(&mut self, past: &[Guess]) -> String;
}

impl Guesser for fn(history: &[Guess]) -> String {
    fn guess(&mut self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
macro_rules! mask {
    (C) => {$crate::Correctness::Correct};
    (M) => {$crate::Correctness::Misplaced};
    (W) => {$crate::Correctness::Wrong};
    ($($c:tt)+) => {[
        $(mask!($c)),+
    ]}
}

#[cfg(test)]
mod tests {
    mod guess_matcher {
        use crate::Guess;
        use std::borrow::Cow;

        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {
                    word: Cow::Borrowed($prev),
                    mask: mask![$($mask )+]
                }.matches($next));
            };
            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {
                    word: Cow::Borrowed($prev),
                    mask: mask![$($mask )+]
                }.matches($next));
            }
        }

        #[test]
        fn matches() {
            check!("abcde" + [C C C C C] allows "abcde");
            check!("abcdf" + [C C C C C] disallows "abcde");
            check!("abcde" + [W W W W W] allows "fghij");
            check!("abcde" + [M M M M M] allows "eabcd");
            check!("aaabb" + [C M W W W] disallows "accaa");
            check!("baaaa" + [W C M W W] allows "aaccc");
            check!("baaaa" + [W C M W W] disallows "caacc");
            check!("abcde" + [W W W W W] disallows "bcdea");
        }
    }

    mod game {
        use crate::{Guess, Guesser, Wordle};

        #[test]
        fn play() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "right".to_string() });
            assert_eq!(w.play("right", guesser), Some(1));
        }

        #[test]
        fn play_2() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(2));
        }

        #[test]
        fn play_3() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(3));
        }
        #[test]
        fn play_4() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return "right".to_string();
                }
                "wrong".to_string()
            });
            assert_eq!(w.play("right", guesser), Some(4));
        }
        #[test]
        fn should_terminate() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "wrong".to_string() });
            assert_eq!(w.play("right", guesser), None);
        }
    }

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
            assert_eq!(
                Correctness::compute("azzaz", "aaabb"),
                [
                    Correctness::Correct,
                    Correctness::Misplaced,
                    Correctness::Wrong,
                    Correctness::Wrong,
                    Correctness::Wrong
                ]
            );
            assert_eq!(
                Correctness::compute("baccc", "aaddd"),
                [
                    Correctness::Wrong,
                    Correctness::Correct,
                    Correctness::Wrong,
                    Correctness::Wrong,
                    Correctness::Wrong
                ]
            );
            assert_eq!(
                Correctness::compute("abcde", "aacde"),
                [
                    Correctness::Correct,
                    Correctness::Wrong,
                    Correctness::Correct,
                    Correctness::Correct,
                    Correctness::Correct
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
