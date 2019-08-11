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

#[test]
fn test_calculate_frequency() {
    let exp = [
        (String::from("aa"), 1),
        (String::from("bb"), 1),
        (String::from("cc"), 1),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();

    // Cursor は内部にバイト列を保持してインメモリバッファを作るデータ型
    let actual = calculate_frequency(std::io::Cursor::new("aa bb cc bb"), CountOption::Word);

    assert_eq!(actual, exp)
}

fn increment(m: &mut HashMap<String, usize>, key: impl ToString) {
    *m.entry(key.to_string()).or_insert(0) += 1;
}

#[test]
fn result_test() -> std::io::Result<()> {
    use std::fs::remove_file;
    remove_file("no_such_file")
}
