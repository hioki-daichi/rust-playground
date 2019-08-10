fn f1(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", f1("World")); // Hello, World!
}
