mod server {
    pub fn echo() {
        println!("Server")
    }
}

mod client {
    pub fn echo() {
        println!("Client")
    }
}

fn main() {
    server::echo();
    client::echo();
}
