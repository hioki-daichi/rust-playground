#[derive(Default, Debug)]
pub struct Polygon<T> {
    pub vertexes: Vec<T>,
}

// 座標
trait Coordinates {}

// デカルト座標
#[derive(Default, Debug)]
struct CartesianCoord {
    x: f64,
    y: f64,
}
impl Coordinates for CartesianCoord {}

// 極座標
#[derive(Default, Debug)]
struct PolarCoord {
    r: f64,
    theta: f64,
}
impl Coordinates for PolarCoord {}

fn main() {
    // デカルト座標
    let v1 = vec![CartesianCoord { x: 0.0, y: 0.0 }];
    let p1 = Polygon {
        vertexes: v1,
        ..Default::default()
    };
    println!("{:?}", p1); // Polygon { vertexes: [CartesianCoord { x: 0.0, y: 0.0 }] }

    // 極座標
    let v2 = vec![PolarCoord { r: 0.0, theta: 0.0 }];
    let p2 = Polygon {
        vertexes: v2,
        ..Default::default()
    };
    println!("{:?}", p2); // Polygon { vertexes: [PolarCoord { r: 0.0, theta: 0.0 }] }
}
