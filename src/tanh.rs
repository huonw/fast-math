/// Calculate the numerator of the `tanh` approximation.
fn a(x: f32) -> f32 {
    let x2 = x * x;
    (((x2 + 378.) * x2 + 17325.) * x2 + 135135.) * x
}

/// Calculate the denominator of the `tanh` approximation.
fn b(x: f32) -> f32 {
    let x2 = x * x;
    ((28. * x2 + 3150.) * x2 + 62370.) * x2 + 135135.
}

/// Compute a fast approximation of the hyperbolic tangent of `x`.
///
/// For large |x|, the output may be outside of [-1, 1].
#[inline]
pub fn tanh_raw(x: f32) -> f32 {
    // Implementation based on
    // https://varietyofsound.wordpress.com/2011/02/14/efficient-tanh-computation-using-lamberts-continued-fraction
    a(x) / b(x)
}

/// Compute a fast approximation of the hyperbolic tangent of `x`.
///
/// See `atanh_raw` for a faster version that may return incorrect results for
/// large `|x|` and `nan`.
#[inline]
pub fn tanh(x: f32) -> f32 {
    if x.is_nan() {
        return x;
    }

    let a = a(x);
    if !a.is_finite() {
        return if a < 0. { -1. } else { 1. };
    }

    let result = a / b(x);
    if result > 1. {
        return 1.;
    }
    if result < -1. {
        return -1.;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck as qc;
    use std::f32 as f;

    /// Maximal absolute error.
    const TOL: f32 = 0.0001;

    #[test]
    fn tanh_abs_err_qc() {
        fn prop(x: f32) -> qc::TestResult {
            let e = tanh(x);
            let t = x.tanh();
            let abs = (e - t).abs();

            qc::TestResult::from_bool(abs < TOL)
        }
        qc::quickcheck(prop as fn(f32) -> qc::TestResult)
    }

    const PREC: u32 = 1 << 20;
    #[test]
    fn tanh_abs_err_exhaustive() {
        for i in 0..PREC + 1 {
            for j in -5..6 {
                let x = (1.0 + i as f32 / PREC as f32) * 2f32.powi(j * 20);
                {
                    let e = tanh(x);
                    let t = x.tanh();
                    let abs = (e - t).abs();

                    assert!(abs < TOL,
                            "{:.8}: {:.8}, {:.8}. {:.4}", x, e, t, abs);
                }
                {
                    let e = tanh(-x);
                    let t = (-x).tanh();
                    let abs = (e - t).abs();

                    assert!(abs < TOL,
                            "{:.8}: {:.8}, {:.8}. {:.4}", -x, e, t, abs);
                }
            }
        }
    }

    #[test]
    fn tanh_edge_cases() {
        assert!(tanh(f::NAN).is_nan());
        assert_eq!(tanh(f::NEG_INFINITY), -1.);
        assert_eq!(tanh(f::INFINITY), 1.);
    }

    #[test]
    fn tanh_denormals() {
        fn prop(x: u8, y: u16) -> bool {
            let signif = ((x as u32) << 16) | (y as u32);
            let mut x = ::float::recompose(0, 1, signif);

            for _ in 0..23 {
                {
                    let e = tanh(x);
                    let t = x.tanh();
                    let abs = (e - t).abs();
                    if abs >= TOL {
                        return false
                    }
                }
                {
                    let e = tanh(-x);
                    let t = (-x).tanh();
                    let abs = (e - t).abs();
                    if abs >= TOL {
                        return false
                    }
                }

                x /= 2.0;
            }
            true
        }
        qc::quickcheck(prop as fn(u8, u16) -> bool)
    }

    #[test]
    fn tanh_raw_denormals() {
        fn prop(x: u8, y: u16) -> bool {
            let signif = ((x as u32) << 16) | (y as u32);
            let mut x = ::float::recompose(0, 1, signif);

            for _ in 0..23 {
                let e = tanh_raw(x);
                let t = x.tanh();
                let abs = (e - t).abs();
                if abs >= TOL {
                    return false
                }

                x /= 2.0;
            }
            true
        }
        qc::quickcheck(prop as fn(u8, u16) -> bool)
    }
}

#[cfg(all(test, feature = "unstable"))]
mod benches {
    use test::{Bencher, black_box};

    const TAB: &'static [f32] =
        &[ 0.85708036,  2.43390621,  2.80163358,  2.55126348,  3.18046186,
           2.88689427,  0.32215155,  0.07701401,  1.22922506,  0.4580259 ,
           0.01257442,  4.23107197,  0.89538113,  1.65219582,  0.14632742,
           1.68663984,  1.88125115,  2.16773942,  1.27461936,  1.03091265];

    #[bench]
    fn tanh(b: &mut Bencher) {
        b.iter(|| {
            for &x in black_box(TAB) {
                black_box(super::tanh(x));
            }
        })
    }

    #[bench]
    fn tanh_raw(b: &mut Bencher) {
        b.iter(|| {
            for &x in black_box(TAB) {
                black_box(super::tanh_raw(x));
            }
        })
    }

    #[bench]
    fn tanh_std(b: &mut Bencher) {
        b.iter(|| {
            for &x in black_box(TAB) {
                black_box(x.tanh());
            }
        })
    }
}
