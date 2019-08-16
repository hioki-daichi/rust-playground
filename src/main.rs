fn f(n: usize) -> Box<[i32]> {
    let f: fn() -> i32 = Default::default;
    println!("{}", f()); // 0

    let a: std::iter::RepeatWith<fn() -> i32> = std::iter::repeat_with(f);
    println!("{:?}", a); // RepeatWith { repeater: 0x10d3b2480 }

    let b: std::iter::Take<std::iter::RepeatWith<fn() -> i32>> = a.take(n);
    println!("{:?}", b); // Take { iter: RepeatWith { repeater: 0x10d3b2480 }, n: 2 }

    let c: std::vec::Vec<i32> = b.collect::<Vec<_>>();

    c.into_boxed_slice()
}

fn main() {
    println!("{:?}", f(2)); // [0, 0]
}
