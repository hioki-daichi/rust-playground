fn main() {
    let a = 12;

    let even_or_odd = if a % 2 == 0 { "an even" } else { "an odd" };

    println!("{} is {} number", a, even_or_odd); // 12 is an even number
}
