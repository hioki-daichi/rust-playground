use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let mut letters = HashMap::new();
    for ch in "dokonokokinoko konokinokodokono dokonokokinoko morinokinoko".chars() {
        let e: Entry<char, i32> = letters.entry(ch);
        let v: &mut i32 = e.or_insert(0);
        *v += 1;
    }
    println!("{:?}", letters); // {'i': 5, ' ': 3, 'o': 23, 'm': 1, 'r': 1, 'd': 3, 'n': 9, 'k': 14}
}
