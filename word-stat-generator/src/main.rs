use color_eyre::Result;
use glob::glob;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

const WORDLE_WORD_SIZE: usize = 5;
const DICTIONARY_WITH_COUNTS: &str = "data/dictionary-with-counts.txt";
const ANSWERS: &str = "data/answers.txt";

fn read_word_dictionary(path: PathBuf) -> Result<HashMap<String, usize>> {
    let input_path = fs::read_to_string(path)?;
    let res = serde_json::from_str::<Vec<String>>(&input_path)?;
    let words = res
        .iter()
        .cloned()
        .map(|s| (s, 0))
        .collect::<HashMap<String, usize>>();
    write_answers_to_file(res, PathBuf::from_str(ANSWERS)?);
    Ok(words)
}

fn write_answers_to_file(mut answers: Vec<String>, path: PathBuf) {
    let file = File::create(path).expect("cannot open file");
    answers.sort();
    for word in answers {
        writeln!(&file, "{word}").expect("writing to a file failed");
    }
}

fn write_words_with_counts_to_file(words: &HashMap<String, usize>, path: PathBuf) {
    let file = File::create(path).expect("cannot open file");
    for (word, count) in words.iter().sorted_by(|a, b| Ord::cmp(a, b)) {
        writeln!(&file, "{word} {count}").expect("writing to a file failed");
    }
}

fn generate_word_statictics(mut words: HashMap<String, usize>) -> Result<()> {
    for entry in glob("data/1-000*.txt")? {
        match entry {
            Ok(path) => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line?;
                    let items: Vec<&str> = line.split('\t').collect();
                    let word = items.first().unwrap();
                    if !word.contains('_') {
                        continue;
                    }
                    let word = word
                        .split('_')
                        .collect::<Vec<&str>>()
                        .first()
                        .unwrap()
                        .to_lowercase();
                    if word.len() == WORDLE_WORD_SIZE && words.contains_key(&word) {
                        *words.get_mut(&word).unwrap() +=
                            items.last().unwrap().split(',').collect::<Vec<&str>>()[1]
                                .parse::<usize>()?
                    }
                }
            }
            Err(e) => println!("{e}"),
        }
    }
    let _ = words.iter_mut().map(|(_, c)| {
        if *c == 0 {
            *c = 1;
        }
    });
    write_words_with_counts_to_file(&words, PathBuf::from_str(DICTIONARY_WITH_COUNTS)?);
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let path = "data/dictionary.json";
    let words = read_word_dictionary(PathBuf::from_str(path)?)?;
    generate_word_statictics(words)?;
    Ok(())
}
