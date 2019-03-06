use ieee754::Ieee754;

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
/// See `tanh_raw` for a faster version that may return incorrect results for
/// large `|x|` and `nan`.
#[inline]
pub fn tanh(x: f32) -> f32 {
    if x.is_nan() {
        return x;
    }

    let a = a(x);
    if !a.is_finite() {
        return 1_f32.copy_sign(a);
    }

    let result = a / b(x);
    if result.abs() > 1. {
        return 1_f32.copy_sign(result);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck as qc;
    use std::f32 as f;
    use ieee754::Ieee754;

    /// Maximal absolute error.
    const TOL_ABS: f32 = 0.0001;

    /// Maximal relative error.
    const TOL_REL: f32 = 0.0001;

    #[test]
    fn tanh_err_qc() {
        fn prop(x: f32) -> qc::TestResult {
            let e = tanh(x);
            let t = x.tanh();
            let abs = (e - t).abs();
            let rel = e.rel_error(t).abs();

            qc::TestResult::from_bool(abs < TOL_ABS && rel < TOL_REL)
        }
        qc::quickcheck(prop as fn(f32) -> qc::TestResult)
    }

    const PREC: u32 = 1 << 20;
    #[test]
    fn tanh_err_exhaustive() {
        for i in 0..PREC + 1 {
            for j in -5..6 {
                let x = (1.0 + i as f32 / PREC as f32) * 2f32.powi(j * 20);
                {
                    let e = tanh(x);
                    let t = x.tanh();
                    let abs = (e - t).abs();
                    let rel = e.rel_error(t).abs();

                    assert!(abs < TOL_ABS,
                            "{:.8}: {:.8}, {:.8}. {:.4}", x, e, t, abs);
                    assert!(rel < TOL_REL,
                            "{:.8}: {:.8}, {:.8}. {:.4}", x, e, t, rel);
                }
                {
                    let e = tanh(-x);
                    let t = (-x).tanh();
                    let abs = (e - t).abs();
                    let rel = e.rel_error(t).abs();

                    assert!(abs < TOL_ABS,
                            "{:.8}: {:.8}, {:.8}. {:.4}", -x, e, t, abs);
                    assert!(rel < TOL_REL,
                            "{:.8}: {:.8}, {:.8}. {:.4}", x, e, t, rel);
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
            let mut x = f32::recompose_raw(false, 1, signif);

            for _ in 0..23 {
                {
                    let e = tanh(x);
                    let t = x.tanh();
                    let abs = (e - t).abs();
                    let rel = e.rel_error(t).abs();
                    if abs >= TOL_ABS && rel >= TOL_REL {
                        return false
                    }
                }
                {
                    let e = tanh(-x);
                    let t = (-x).tanh();
                    let abs = (e - t).abs();
                    let rel = e.rel_error(t).abs();
                    if abs >= TOL_ABS && rel >= TOL_REL {
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
            let mut x = f32::recompose_raw(false, 1, signif);

            for _ in 0..23 {
                let e = tanh_raw(x);
                let t = x.tanh();
                let abs = (e - t).abs();
                let rel = e.rel_error(t).abs();
                if abs >= TOL_ABS && rel >= TOL_REL {
                    return false
                }

                x /= 2.0;
            }
            true
        }
        qc::quickcheck(prop as fn(u8, u16) -> bool)
    }
}
