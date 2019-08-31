use std::str::FromStr;

trait Server {
    type Response;
    type Request: FromStr; // FromStr がトレイト境界になる。

    fn handle(&self, req: Self::Request) -> Self::Response;
}

struct EchoServer;

impl Server for EchoServer {
    type Response = String;
    type Request = String; // トレイト境界
    fn handle(&self, req: Self::Request) -> Self::Response {
        req
    }
}

fn main() {
    let es = EchoServer {};
    let res = es.handle("REQUEST".to_string());
    println!("{:?}", res); // "REQUEST"
}
