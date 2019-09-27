// use failure::{Error, Fail};
// use std::error::Error as StdError;
//
// fn main() {}
//
// #[derive(Fail, Debug)]
// pub enum ErrorKind {
//     #[fail(display = "foo error")]
//     Foo,
// }
//
use std::io;
use std::io::BufRead;

use failure::{format_err, Error};

fn my_function() -> Result<(), Error> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?;

        if line.chars().all(|c| c.is_whitespace()) {
            break;
        }

        if !line.starts_with("$") {
            return Err(format_err!("Input did not begin with `$`"));
        }

        println!("{}", &line[1..]);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let result = my_function()?;
    println!("{:?}", result);
    Ok(())
}
