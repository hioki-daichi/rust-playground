use regex::Regex;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::string::String;

const ERR_FILENAME_ARGUMENT: &str = "filename is required for argument.";
const ERR_FILE_OPEN: &str = "failed to open the file with the specified name.";

fn main() {
    let filename = get_filename();
    let file = File::open(filename).expect(ERR_FILE_OPEN);
    let reader = BufReader::new(&file);
    let freqs = count(reader);
    println!("{:?}", freqs);
}

fn get_filename() -> String {
    let mut args: env::Args = env::args();
    let filename: Option<String> = args.nth(1);
    return filename.expect(ERR_FILENAME_ARGUMENT);
}

fn count(input: impl BufRead) -> HashMap<String, usize> {
    let word_regex = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        for m in word_regex.find_iter(&line) {
            let word = m.as_str().to_string();
            *freqs.entry(word).or_insert(0) += 1;
        }
    }

    freqs
}
