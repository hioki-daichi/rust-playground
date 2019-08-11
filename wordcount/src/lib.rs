use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

pub enum CountOption {
    Char,
    Word,
    Line,
}

// デフォルトは Word にしておく
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

pub fn calculate_frequency(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    use crate::CountOption::*;
    let mut m = HashMap::new();
    for line in input.lines() {
        let line = line.unwrap();

        match option {
            Char => {
                for c in line.chars() {
                    increment(&mut m, c);
                }
            }
            Word => {
                for mat in Regex::new(r"\w+").unwrap().find_iter(&line) {
                    increment(&mut m, mat.as_str());
                }
            }
            Line => {
                increment(&mut m, line);
            }
        }
    }
    m
}

fn increment(m: &mut HashMap<String, usize>, key: impl ToString) {
    *m.entry(key.to_string()).or_insert(0) += 1;
}
