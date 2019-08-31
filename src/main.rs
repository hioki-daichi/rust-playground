mod some_module;

use some_module::Coordinates;
use some_module::LinearTransform;

fn main() {
    let c = (1.0, 0.0).to_cartesian();
    let m = some_module::Matrix([[1.0, 2.0], [3.0, 4.0]]);
    println!("{:?}", c.transform(&m)); // CartesianCoord { x: 1.0, y: 3.0 }
}
