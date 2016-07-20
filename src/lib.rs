
pub mod one_dim {
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

    #[test]
    fn test_newton() {
        let mut x = -5.0;
        let dx = 1e-7;
        let f = |x| x * (x - 1.0);
        let df = |x| (f(x + dx) - f(x)) / dx;
        let eps = 1e-6;
        x = newton(&f, &df, x, eps);
        assert!(f(x).abs() < eps);
    }
}
