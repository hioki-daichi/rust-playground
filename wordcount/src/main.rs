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
    let frequency = calculate_frequency(reader);
    println!("{:?}", frequency);
}

fn get_filename() -> String {
    let mut args: env::Args = env::args();
    let filename: Option<String> = args.nth(1);
    return filename.expect(ERR_FILENAME_ARGUMENT);
}

fn calculate_frequency(input: impl BufRead) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for line in input.lines() {
        for mat in Regex::new(r"\w+").unwrap().find_iter(&line.unwrap()) {
            // or_insert(0) では、マッチした word に該当する HashMap の Entry の Value に初期値 0 を設定しつつミュータブルな参照を取得している。
            // そしてそのミュータブルな参照をさらに * で Dereference して実体の usize のデータ取得しインクリメントしている。
            *m.entry(mat.as_str().to_string()).or_insert(0) += 1;
        }
    }
    m
}
