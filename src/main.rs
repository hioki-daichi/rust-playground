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

// 関連型が特定の型を持っていることを指定したい場合は `トレイト名<関連型名 = 型>` のように指定できる。
// `Server<Request = String>` のように書いた場合、Request に String を持つ Server の実装しか受け付けない。
fn handle<S: Server<Request = String>>(server: S, req: &str) -> S::Response {
    server.handle(req.to_string())
}

fn main() {
    println!("{:?}", handle(EchoServer {}, "REQUEST")); // "REQUEST"
}
