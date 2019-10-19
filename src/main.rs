fn foo(x: &i32, y: &i32) {
    println!("{:?} {:?}", x, y);
}

fn bar<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("{:?} {:?}", x, y);
}

fn main() {
    foo(&1, &2);
    bar(&1, &2);
}
