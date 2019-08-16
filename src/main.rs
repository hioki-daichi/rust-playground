fn main() {
    let f: fn() -> i32 = Default::default;
    println!("{}", f()); // 0

    let a: std::iter::RepeatWith<fn() -> i32> = std::iter::repeat_with(f);
    println!("{:?}", a); // RepeatWith { repeater: 0x10d3b2480 }

    let b: std::iter::Take<std::iter::RepeatWith<fn() -> i32>> = a.take(2);
    println!("{:?}", b); // Take { iter: RepeatWith { repeater: 0x10d3b2480 }, n: 2 }

    let c: std::vec::Vec<i32> = b.collect::<Vec<_>>();
    println!("{:?}", c); // [0, 0]

    let d: std::boxed::Box<[i32]> = c.into_boxed_slice();
    println!("{:?}", d); // [0, 0]
}
