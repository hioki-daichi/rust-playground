#[derive(Debug, PartialEq)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

fn foo(weekday: Weekday) -> String {
    if weekday == Weekday::Friday {
        return format!("{:?}!!!", weekday);
    } else {
        return format!("still {:?}...", weekday);
    }
}

fn main() {
    println!("{:?}", foo(Weekday::Monday));
    println!("{:?}", foo(Weekday::Tuesday));
    println!("{:?}", foo(Weekday::Wednesday));
    println!("{:?}", foo(Weekday::Thursday));
    println!("{:?}", foo(Weekday::Friday));
    // "still Monday..."
    // "still Tuesday..."
    // "still Wednesday..."
    // "still Thursday..."
    // "Friday!!!"
}
