use std::collections::HashMap;

// 指定した k が存在しない場合: (k, "?") のように "?" を Value に持つ Entry を作成する。
// 指定した k が存在する場合:   (k, "<元のValue>!") のように "!" を追記する。
fn f(k: char, m: &mut HashMap<char, String>) {
    // get_mut の第1引数は &mut self なので m の可変の借用をとっている
    let o: Option<&mut String> = m.get_mut(&k);

    // o (可変の参照) が生存している間は m の可変の借用が有効。
    // m に対する可変の借用が 2 つ同時に存在してはいけない。

    match o {
        // Some(s) の腕では s を戻り値に束縛しているためこの腕の中まで m の可変の借用が有効になる。
        Some(s) => s.push_str("!"),
        // None の腕では戻り値に何も束縛しておらず get_mut による m の可変の借用は終了している
        // ため insert は問題なく m の可変の借用をとれる
        None => {
            // insert の第1引数は &mut self なので m の可変の借用をとっている
            m.insert(k, String::from("?"));
        }
    }
}

fn main() {
    let mut m = HashMap::new();

    f('a', &mut m);
    println!("{:?}", m); // {'a': "?"}

    f('a', &mut m);
    println!("{:?}", m); // {'a': "?!"}

    f('a', &mut m);
    println!("{:?}", m); // {'a': "?!!"}
}
