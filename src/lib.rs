
pub fn newton<T1, T2>(f: T1, df: T2, mut x: f64, eps: f64) -> f64
    where T1: Fn(f64) -> f64,
          T2: Fn(f64) -> f64
{
    let mut y = f(x);
    while y.abs() > eps {
        x -= y / df(x);
        y = f(x);
    }
    x
}
