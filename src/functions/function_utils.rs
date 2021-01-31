macro_rules! polynomial {
    ($x:ident, $poly:expr) => {
        {
            let mut iter = $poly.iter().rev();
            let init = iter.next().unwrap();
            iter.fold(*init, |a, n| $x * a + n)
        }
    };
    ($x:ident, $coeff:expr, $($coeffs:expr),+) => (
        $x * polynomial!($x, $($coeffs),+) + $coeff
    );
}
