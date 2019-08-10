mod shape {
    #[derive(Debug, Default)]
    pub struct Polygon {
        pub vertexes: Vec<(i32, i32)>,
        pub stroke_width: u8,
        pub fill: (u8, u8, u8),
        internal_id: String,
    }

    pub fn new_polygon(vertexes: Vec<(i32, i32)>, stroke_width: u8, fill: (u8, u8, u8)) -> Polygon {
        Polygon {
            vertexes,
            stroke_width,
            fill,
            internal_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
        }
    }
}

use shape::new_polygon;

fn main() {
    let p = new_polygon(vec![(0, 0)], 1, (0, 0, 0));

    println!("{:?}", p.vertexes); // [(0, 0)]
    println!("{:?}", p.stroke_width); // 1
    println!("{:?}", p.fill); // (0, 0, 0)
    println!("{:?}", p.internal_id); // field `internal_id` of struct `shape::Polygon` is private
}
