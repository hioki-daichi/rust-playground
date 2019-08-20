fn main() {
    let s1: String = "read-only".to_string();
    let lookup = || s1.find('d').is_some();
    let _handle = std::thread::spawn(lookup);
}
