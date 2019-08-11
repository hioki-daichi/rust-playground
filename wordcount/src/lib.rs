use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

pub fn calculate_frequency(input: impl BufRead) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for line in input.lines() {
        for mat in Regex::new(r"\w+").unwrap().find_iter(&line.unwrap()) {
            *m.entry(mat.as_str().to_string()).or_insert(0) += 1;
        }
    }
    m
}
