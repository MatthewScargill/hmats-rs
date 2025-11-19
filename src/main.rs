//mod kernels;
//use kernels::*;

use hmats_rs::*;

fn main() {
    let x = [0.0_f64, 0.0];
    let y = [4.0_f64, 0.0];
    let laplace = Laplace2D;
    let val = laplace.eval(&x, &y);
    println!("Laplace Greens function = {:?}", val);
}
