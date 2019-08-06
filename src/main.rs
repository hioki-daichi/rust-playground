use std::collections::HashMap;

// get_mut と insert の 2 つのメソッドで map に対して可変の借用をとっている。
// 可変の借用が 2 つ同時に存在することは許されないため、借用のライフタイムが重ならないようにしなけ
// ればならない。
fn process_or_default(key: char, map: &mut HashMap<char, String>) {
    // get_mut は Option<&mut String> を返す。
    // この戻り値(可変の参照)が生存している間は map の可変の借用が有効。
    match map.get_mut(&key) {
        // value を戻り値に束縛しているためこの腕の中まで map の可変の借用が有効になる。
        Some(value) => value.push_str(", world!"),
        // None 側は何も束縛していないため get_mut による可変の借用は終了している
        None => {
            map.insert(key, Default::default()); // 再び insert で可変の借用をとれる
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
    println!("{:?}", map); //=> {'h': "Hello, world!"}
}
