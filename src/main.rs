#[derive(Debug)]
struct Parent(u32, Child, Child);

#[derive(Debug)]
struct Child(u32);

fn main() {
    // Child(11) と Child(12) の 2 つ値を作り、それらを Parent(1, ..) にもたせていることから
    // Child(11) と Child(12) の所有者は Parent(1, ..) になる。
    // (Parent(1, ..) のような値自身も別の値の所有者になれる。)
    //
    // また、Parent(1, ..) は変数 p に持たせているため
    // Parent(1, ..) の所有者は p になる。
    let p = Parent(1, Child(11), Child(12));
    println!("{:?}", p);
}
