use dataloader::{BatchFn, BatchFuture, Loader};
use futures::{executor, future, FutureExt as _, TryFutureExt as _};

struct Batcher;

impl BatchFn<i32, i32> for Batcher {
    type Error = ();

    fn load(&self, xs: &[i32]) -> BatchFuture<i32, Self::Error> {
        let f = |v| v * 10;
        future::ready(xs.into_iter().map(f).collect())
            .unit_error()
            .boxed()
    }
}

fn main() {
    test_loader(1, vec![8, 10]);
}

fn test_loader(n: i32, expected: Vec<i32>) {
    let loader = Loader::new(Batcher);

    let load_three_times = loader
        .load(n)
        .and_then(|v| loader.load_many(vec![v, v + 1]));

    let actual = executor::LocalPool::new()
        .run_until(load_three_times)
        .unwrap();

    assert_eq!(actual, expected);
}

// fn test_cached_loader() {
//     let loader = Loader::new(Batcher).cached();
//     let f = |v| loader.load_many(vec![v, v + 1, v + 2]);
//     let v1 = loader.load(1).and_then(f);
//     let v2 = loader.load(2).and_then(f);
//     let v3 = loader.load(3).and_then(f);
//
//     let actual = executor::LocalPool::new()
//         .run_until(future::try_join3(v1, v2, v3))
//         .unwrap();
//
//     let expected = (vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]);
//
//     assert_eq!(expected, actual);
// }
