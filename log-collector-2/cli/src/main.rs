struct ApiClient {
    server: String,
    client: reqwest::Client,
}

impl ApiClient {
    fn post_logs() {unimplemented!}
    fn get_logs() {unimplemented!}
    fn get_csv() {unimplemented!}
}

fn main() {
    // TODO: SubCommand::with_name で get の定義
    // TODO: SubCommand::with_name で post の定義
    // TODO: clap::App::new して get と post を subcommand に登録する
    // TODO: opts を parse する
    // TODO: parse された情報から SERVER の値を抜き出して ApiClient の生成
    // TODO: parse された subcommand を抜き出して get にマッチしたら get を呼び、post にマッチしたら post を呼ぶ
    // TODO: get なら FORMAT を抜き出し Format 構造体に map して CSV なら CSV を、JSON なら JSON を返す
    // TODO: post なら post_csv を呼ぶ
}
