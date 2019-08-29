#[derive(Debug)]
struct CartesianCoord {
    x: f64,
    y: f64,
}

trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
}

#[allow(dead_code)]
fn print_point<P: Coordinates>(point: P) {
    let cart = point.to_cartesian();
    println!("({} {})", cart.x, cart.y);
}

impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }
}

impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }
}

struct Matrix([[f64; 2]; 2]);

// Coordinates トレイトを継承している。
// 座標に対して線形変換を定義している。
trait LinearTransform: Coordinates {
    fn transform(self, matrix: &Matrix) -> Self;

    // デフォルト実装を持てる。
    fn rotate(self, theta: f64) -> Self
    where
        Self: Sized,
    {
        self.transform(&Matrix([
            [theta.cos(), -theta.sin()],
            [theta.sin(), theta.cos()],
        ]))
    }
}

// CartesianCoord に LinearTransform を実装する
impl LinearTransform for CartesianCoord {
    fn transform(mut self, matrix: &Matrix) -> Self {
        let x = self.x;
        let y = self.y;
        let m = matrix.0;

        self.x = m[0][0] * x + m[0][1] * y;
        self.y = m[1][0] * x + m[1][1] * y;
        self
    }
}

fn main() {
    println!("{:?}", (1.0, 0.0).to_cartesian()); // CartesianCoord { x: 1.0, y: 0.0 }

    println!(
        "{:?}",
        (1.0, 0.0).to_cartesian().rotate(std::f64::consts::PI)
    ); // CartesianCoord { x: -1.0, y: 0.00000000000000012246467991473532 }
}
