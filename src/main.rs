trait SomeTrait {
    fn take_ref(&self);
}

impl SomeTrait for str {
    fn take_ref(&self) {}
}

fn main() {
    let s = "hoge";
    s.take_ref();

    // Box<str> に対しても呼べると本には書いてあったがコンパイルエラー
    let box_s = Box::new(*s); // the size for values of type `str` cannot be known at compilation time    doesn't have a size known at compile-time    help: the trait `std::marker::Sized` is not implemented for `str`

    box_s.take_ref();
}
