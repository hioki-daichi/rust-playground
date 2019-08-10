#[derive(Debug)]
struct Polygon {
    vertexes: Vec<(i32, i32)>, // 頂点の座標
    stroke_width: u8,          // 輪郭の太さ
    fill: (u8, u8, u8),        // 塗りつぶしの RGB 色
}

fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
    let stroke_width = 1;
    let fill = (0, 0, 0);
    Polygon {
        vertexes,
        stroke_width,
        fill,
    } // 初期化略記構文 (field init shorthand syntax)
}

fn main() {
    let triangle = new_polygon(vec![(0, 0), (3, 0), (2, 2)]);

    println!("{:?}", triangle); // Polygon { vertexes: [(0, 0), (3, 0), (2, 2)], stroke_width: 1, fill: (0, 0, 0) }

    // フィールド名でアクセス
    assert_eq!(triangle.vertexes[0], (0, 0));

    // パターンマッチでアクセス。不要なフィールドは .. で省略。
    let Polygon {
        stroke_width: sw, ..
    } = triangle;
    assert_eq!(sw, 1);

    // : 以降を省略するとフィールド名と同じ名前の変数が作られる。
    let Polygon { fill, .. } = triangle;
    assert_eq!(fill, (0, 0, 0));

    // stroke_width だけ異なる新しい値を作る
    let triangle2 = Polygon {
        stroke_width: 2,
        ..triangle // レコードアップデート構文 (functional record update syntax)
    };
    println!("{:?}", triangle2); // Polygon { vertexes: [(0, 0), (3, 0), (2, 2)], stroke_width: 2, fill: (0, 0, 0) }
}
