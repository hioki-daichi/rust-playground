use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

fn main() -> Result<(), Box<dyn Error>> {
    let a: HashSet<_> = ["foo", "bar"].iter().cloned().collect();
    let a = Arc::new(RwLock::new(a));

    fn stringify(a: impl ToString) -> String {
        a.to_string()
    }

    {
        let b = a.read().map_err(stringify)?; // read ロックを取得している
        assert!(b.contains("foo"));
        assert!(b.contains("bar"));
    } // b がスコープをはずれロックが解除される

    a.write().map_err(stringify)?.insert("baz");

    let c = Arc::clone(&a);

    std::thread::spawn(move || c.write().map(|mut d| d.insert("qux")).map_err(stringify))
        .join()
        .expect("Thread error")?;

    assert!(a.read().map_err(stringify)?.contains("baz"));
    assert!(a.read().map_err(stringify)?.contains("qux"));

    Ok(())
}
