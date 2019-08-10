fn main() {
    // char の並びから str は作れないが String なら作れる。
    let cs: [char; 4] = ['R', 'u', 's', 't'];
    let s1: String = cs.iter().collect::<String>();
    assert_eq!(s1, "Rust");
}
