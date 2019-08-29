#[derive(Debug)]
struct CartesianCoord {
    x: f64,
    y: f64,
}

struct Matrix([[f64; 2]; 2]);

trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
}

trait LinearTransform {
    fn transform(self, matrix: &Matrix) -> Self;
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

impl LinearTransform for CartesianCoord {
    fn transform(self, matrix: &Matrix) -> Self {
        let mut cart = self.to_cartesian();
        println!("{:?}", cart);
        let x = cart.x;
        let y = cart.y;
        let m = matrix.0;

        cart.x = m[0][0] * x + m[0][1] * y;
        cart.y = m[1][0] * x + m[1][1] * y;
        cart
    }
}

fn main() {
    let c: CartesianCoord = (1.0, 0.0).to_cartesian();
    let m: Matrix = Matrix([[2.0, 3.0], [4.0, 5.0]]);

    println!("{:?}", c.transform(&m)); // CartesianCoord { x: 2.0, y: 4.0 }
}
