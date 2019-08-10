#[repr(transparent)]
#[derive(Debug)]
struct Id(u64);

#[repr(transparent)]
#[derive(Debug)]
struct Name(String);

#[repr(transparent)]
#[derive(Debug)]
struct Timestamp(u64);

type User = (Id, Name, Timestamp);

fn new_user(name: Name, id: Id, created: Timestamp) -> User {
    (id, name, created)
}

fn main() {
    let id = Id(1);
    let created = Timestamp(9999999999);
    let name = Name(String::from("daichi"));

    let user = new_user(name, id, created);

    println!("{:?}", user);
}
