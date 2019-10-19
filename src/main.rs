fn foo(x: &i32, y: &i32) {
    println!("{:?} {:?}", x, y);
}

fn bar<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("{:?} {:?}", x, y);
}

fn baz<'a, 'b>(x: &'a mut i32, y: &'b i32) -> i32 {
    *x += y;
    *x
}

fn main() {
    foo(&1, &2);
    bar(&1, &2);
    let mut a: i32 = 100;
    baz(&mut a, &5);
    println!("{:?}", a);
}
