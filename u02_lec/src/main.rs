/* use nalgebra::{Matrix2, Vector2};

fn main() {
    let s = Vector2::new(1.0, 0.0).transpose();
    let p = Matrix2::new(0.8, 0.2, 0.3, 0.7);
    let step = s*p;
    println!("Probabilities after one step: {}", step);
} */

use nalgebra::{Matrix4, Vector4, U2, U3};
fn main() {
    let s = Vector4::new(1.0, 0.0, 0.0, 0.0).transpose();
    let p = Matrix4::new(
        0.6, 0.2, 0.15, 0.05, 0.64, 0.16, 0.16, 0.04, 0.45, 0.15, 0.3, 0.1, 0.48, 0.12, 0.32, 0.08,
    );
    let mut p_lim = s;
    for _ in 0..100 {
        p_lim = p_lim*p;
    }
    println!("Final probabilities: {}", p_lim);
}
