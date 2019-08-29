use regex::Regex;

use std::collections::HashMap;
use std::io::BufRead;
use std::string::String;

pub fn calculate_frequency(input: impl BufRead) -> HashMap<String, usize> {
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
