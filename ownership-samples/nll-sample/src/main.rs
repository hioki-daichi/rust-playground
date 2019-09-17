use std::collections::HashMap;

fn f(c: char, m: &mut HashMap<char, String>) {
    let opt_mut_string: Option<&mut String> = m.get_mut(&c);
    match opt_mut_string {
        Some(mut_string) => mut_string.push_str("A"),
        None => {
            m.insert(c, String::from("B"));
        }
    }
}

fn main() {
    let mut m = HashMap::new();
    m.insert('a', "A".to_string());
    f('b', &mut m);
    println!("{:?}", m); // {'a': "A", 'b': "B"}
    f('a', &mut m);
    println!("{:?}", m); // {'a': "AA", 'b': "B"}
}
