use std::f32::consts::PI;

/// Compute a fast approximation of the inverse tangent for `|x| < 1`.
///
/// This will return unspecified nonsense if `x` is doesn't not
/// satisfy those constraints. Use `atan2` if correct handling is
/// required (at the expense of some speed).
#[inline]
pub fn atan_raw(x: f32) -> f32 {
    // Quadratic approximation recommended in
    // http://www-labs.iro.umontreal.ca/~mignotte/IFT2425/Documents/EfficientApproximationArctgFunction.pdf.
    const N1: f32 = PI / 4.;
    const N2: f32 = 0.273;
    (N1 + N2 - N2 * x.abs()) * x
}

/// Compute a fast approximation of the arctangent of `x`.
///
/// The maximum absolute error across all f32s is less than 0.0038.
///
/// See also `atan_raw` which only works on `|x| <= 1`, but is faster.
#[inline]
pub fn atan(x: f32) -> f32 {
    if x.abs() <= 1. {
        atan_raw(x)
    } else {
        x.signum() * PI / 2. - atan_raw(1./x)
    }
}

/// Compute a fast approximation of the four quadrant arctangent of `y` and `x`.
///
/// The maximum absolute error across all f32s is less than 0.0038.
#[inline]
pub fn atan2(y: f32, x: f32) -> f32 {
    if x == 0. {
        if y > 0. {
            return PI / 2.;
        } else if y < 0. {
            return -PI / 2.;
        } else if y.is_nan() {
            return y;
        }
        return match (y.is_sign_positive(), x.is_sign_positive()) {
            (true, true) => 0.,
            (true, false) => PI,
            (false, true) => -0.,
            (false, false) => -PI,
        };
    }
    if x.abs() > y.abs() {
        let z = if y.is_finite() || x.is_finite() {
            y / x
        } else {
            y.signum() * x.signum()
        };
        if x > 0. {
            atan_raw(z)
        } else if y >= 0. {
            atan_raw(z) + PI
        } else {
            atan_raw(z) - PI
        }
    } else {
        // Use `atan(1/x) == sign(x) * pi / 2 - atan(x).
        let z = if x.is_finite() || y.is_finite() {
            x / y
        } else {
            x.signum() * y.signum()
        };
        if y > 0. {
            -atan_raw(z) + PI / 2.
        } else {
            -atan_raw(z) - PI / 2.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck as qc;
    use std::f32 as f;

    /// Maximal absolute error according to paper.
    const TOL: f32 = 0.0038;

    #[test]
    fn atan_abs_err_qc() {
        fn prop(x: f32) -> qc::TestResult {
            let e = atan(x);
            let t = x.atan();
            let abs = (e - t).abs();

            if x == 0.0 {
                qc::TestResult::from_bool(e == 0.0)
            } else {
                qc::TestResult::from_bool(abs < TOL)
            }
        }
        qc::quickcheck(prop as fn(f32) -> qc::TestResult)
    }

    const PREC: u32 = 1 << 20;
    #[test]
    fn atan_abs_err_exhaustive() {
        for i in 0..PREC + 1 {
            for j in -5..6 {
                let x = (1.0 + i as f32 / PREC as f32) * 2f32.powi(j * 20);
                let e = atan(x);
                let t = x.atan();
                let abs = (e - t).abs();

                assert!((e == 0. && x == 0.) || abs < TOL,
                        "{:.8}: {:.8}, {:.8}. {:.4}", x, e, t, abs);
            }
        }
    }

    #[test]
    fn atan_edge_cases() {
        assert!(atan(f::NAN).is_nan());
        assert_eq!(atan(f::NEG_INFINITY), -PI / 2.);
        assert_eq!(atan(0.), 0.);
        assert_eq!(atan(f::INFINITY), PI / 2.);
    }

    #[test]
    fn atan_denormals() {
        fn prop(x: u8, y: u16) -> bool {
            let signif = ((x as u32) << 16) | (y as u32);
            let mut x = ::float::recompose(0, 1, signif);

            for _ in 0..23 {
                assert!(x > 0.0);
                let e = atan(x);
                let t = x.atan();
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

    #[test]
    fn atan2_abs_err_qc() {
        fn prop(y: f32, x: f32) -> qc::TestResult {
            let e = atan2(y, x);
            let t = y.atan2(x);
            let abs = (e - t).abs();

            qc::TestResult::from_bool(abs < 0.0038)
        }
        qc::quickcheck(prop as fn(f32, f32) -> qc::TestResult)
    }

    #[test]
    fn atan2_edge_cases() {
        assert_eq!(atan2(0., 0.), 0.);

        assert_eq!(atan2(0., 0.), 0.);
        assert_eq!(atan2(0., -0.), PI);
        assert_eq!(atan2(-0., 0.), -0.);
        assert_eq!(atan2(-0., -0.), -PI);

        for &v in &[-1., 0., 1., f::INFINITY, f::NEG_INFINITY] {
            assert!(atan2(f::NAN, v).is_nan());
            assert!(atan2(v, f::NAN).is_nan());
        }
        assert!(atan2(f::NAN, f::NAN).is_nan());

        assert!((atan2(f::INFINITY, f::INFINITY) - 0.25 * PI).abs() < TOL);
        assert!((atan2(f::INFINITY, f::NEG_INFINITY) - 0.75 * PI).abs() < TOL);
        assert!((atan2(f::NEG_INFINITY, f::INFINITY) + 0.25 * PI).abs() < TOL);
        assert!((atan2(f::NEG_INFINITY, f::NEG_INFINITY) + 0.75 * PI).abs() < TOL);
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
    fn atan(b: &mut Bencher) {
        b.iter(|| {
            for &x in black_box(TAB) {
                black_box(super::atan(x));
            }
        })
    }

    #[bench]
    fn atan_raw(b: &mut Bencher) {
        b.iter(|| {
            for &x in black_box(TAB) {
                black_box(super::atan_raw(x));
            }
        })
    }

    #[bench]
    fn atan_std(b: &mut Bencher) {
        b.iter(|| {
            for &x in black_box(TAB) {
                black_box(x.atan());
            }
        })
    }
}
