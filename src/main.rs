#[derive(Debug)]
struct Polygon {
    vertexes: Vec<(i32, i32)>, // 頂点の座標
    stroke_width: u8,          // 輪郭の太さ
    fill: (u8, u8, u8),        // 塗りつぶしの RGB 色
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            stroke_width: 1,
            vertexes: Default::default(),
            fill: Default::default(),
        }
    }
}

fn main() {
    let p = Polygon {
        ..Default::default()
    };

    println!("{:?}", p); // Polygon { vertexes: [], stroke_width: 1, fill: (0, 0, 0) }
}
