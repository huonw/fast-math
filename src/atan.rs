use core::f32::INFINITY;
use core::f32::consts::{PI, FRAC_PI_2, FRAC_PI_4};
use float::{flip_sign_nonnan};
use ieee754::Ieee754;

/// Compute a fast approximation of the inverse tangent for `|x| < 1`.
///
/// This will return unspecified nonsense if `x` is doesn't not
/// satisfy those constraints. Use `atan2` if correct handling is
/// required (at the expense of some speed).
#[inline]
pub fn atan_raw(x: f32) -> f32 {
    // Quadratic approximation recommended in
    // http://www-labs.iro.umontreal.ca/~mignotte/IFT2425/Documents/EfficientApproximationArctgFunction.pdf.
    const N2: f32 = 0.273;
    (FRAC_PI_4 + N2 - N2 * x.abs()) * x
}

/// Compute a fast approximation of the arctangent of `x`.
///
/// The maximum absolute error across all f32s is less than 0.0038.
///
/// See also `atan_raw` which only works on `|x| <= 1`, but is faster.
#[inline]
pub fn atan(x: f32) -> f32 {
    if x.abs() > 1.0 {
        // if x is NaN, abs(x) is NaN, so the comparison can't succeed
        debug_assert!(!x.is_nan());
        flip_sign_nonnan(x, FRAC_PI_2) - atan_raw(1./x)
    } else {
        atan_raw(x)
    }
}

/// Compute a fast approximation of the four quadrant arctangent of `y` and `x`.
///
/// The maximum absolute error across all f32s is less than 0.0038.
#[inline]
pub fn atan2(y: f32, x: f32) -> f32 {
    if y.abs() < x.abs() {
        // x is not NaN and y is finite, so there should be no NaNs
        // around
        debug_assert!(!x.is_nan() && !y.is_nan() && !(y / x).is_nan());

        let bias = if x > 0.0 { 0.0 } else { PI };
        flip_sign_nonnan(y, bias) + atan_raw(y / x)
    } else if x == 0. {
        // x is non-NaN
        if y == 0. {
            let bias = if x.is_sign_positive() { 0.0 } else { PI };
            flip_sign_nonnan(y, bias)
        } else if y.is_nan() {
            y
        } else {
            FRAC_PI_2.copy_sign(y)
        }
    } else if y.abs() == INFINITY && x.abs() == INFINITY {
        // x and y are both infinite, meaning: not NaN, can't be
        // divided, and the answer is statically obvious (some
        // multiple of PI/4).
        flip_sign_nonnan(y, FRAC_PI_2 - flip_sign_nonnan(x, FRAC_PI_4))
    } else {
        // Either one x or y is NaN (propogates through atan_raw
        // properly), or |y| >= |x| (meaning |r| = |y / x| >= 1). Use
        // `atan(1/r) == sign(r) * pi / 2 - atan(r)`, but inline the 0
        // or PI `x` bias.
        flip_sign_nonnan(y, FRAC_PI_2) - atan_raw(x / y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck as qc;
    use std::f32 as f;
    use ieee754::Ieee754;

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
            let mut x = f32::recompose_raw(false, 1, signif);

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
        let values = &[-2., -1., -0., 0., 1., 2., f::INFINITY, f::NEG_INFINITY, f::NAN];
        for &x in values {
            for &y in values {
                let e = atan2(x, y);
                let t = x.atan2(y);
                assert_eq!(e.is_nan(), t.is_nan());
                if !t.is_nan() {
                    assert!((e - t).abs() < TOL ||
                            (e - t - 2.0 * PI).abs() < TOL ||
                            (e - t + 2.0 * PI).abs() < TOL);
                }
            }
        }
    }
}
