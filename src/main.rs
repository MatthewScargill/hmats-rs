//mod kernels;
//use kernels::*;

use hmats_rs::*;

fn main() {
    let x = [0.0_f64, 0.0];
    let y = [3.0_f64, 0.0];
    let val = Laplace2D::eval(&x, &y);
    println!("Laplace Greens function = {}", val);
}
