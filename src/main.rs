
extern crate newton;

use newton::one_dim::newton;

fn main() {
    let mut x = -5.0;
    let dx = 1e-7;
    let f = |x| x * (x - 1.0);
    let df = |x| (f(x + dx) - f(x)) / dx;
    let eps = 1e-6;
    x = newton(&f, &df, x, eps);
    println!("Result = {:e}", x);
}
