// struct Foo<T>(T);
// struct Bar<T: ?Sized>(T);
//
// // the size for values of type `[i32]` cannot be known at compilation time    doesn't have a size known at compile-time    help: the trait `std::marker::Sized` is not implemented for `[i32]`  note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
// // struct FooUse(Foo<[i32]>);
// struct BarUse(Bar<[i32]>);

trait Foo {}
trait Bar: Sized {}

struct Impl;
impl Foo for Impl {}
impl Bar for Impl {}

let x: &dyn Foo = &Impl;
let x: &dyn Bar = &Impl;
