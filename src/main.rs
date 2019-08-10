type UserName = String;
type Id = i64;
type Timestamp = i64;
type User = (Id, UserName, Timestamp);

fn new_user(name: UserName) -> User {
    (1, name, 5555555555)
}

fn main() {
    println!("{:?}", new_user(String::from("daichi"))); // (1, "daichi", 5555555555)
}
