use float;
use core::f32 as f;

/// Compute a fast approximation of the base-2 logarithm of `x`.
///
/// The maximum relative error across all positive f32s (including
/// denormals) is less than 0.022. The maximum absolute error is less
/// than 0.009.
///
/// If `x` is negative, or NaN, `log2` returns `NaN`.
///
/// See also `log2_raw` which only works on positive, finite,
/// non-denormal floats, but is 30-40% faster.
///
///
/// |               | Time (ns) |
/// |--------------:|----------------|
/// |    `x.log2()` | 14.3           |
/// |     `log2(x)` | 4.0            |
/// | `log2_raw(x)` | 2.7            |
#[inline]
pub fn log2(x: f32) -> f32 {
    let (sign, exp, signif) = float::decompose(x);
    if sign == 1 {
        f::NAN
    } else if exp == 0 {
        log2_exp_0(signif)
    } else if exp == 0xFF {
        if signif == 0 {
            f::INFINITY
        } else {
            f::NAN
        }
    } else {
        log2_raw(x)
    }
}

#[inline(never)]
fn log2_exp_0(signif: u32) -> f32 {
    if signif == 0 {
        f::NEG_INFINITY
    } else {
        // denormal
        let zeros = signif.leading_zeros() - 9;
        -126.0 - zeros as f32 + log2(float::recompose(0, 127, signif << zeros))
    }
}

/// Compute a fast approximation of the base-2 logarithm of **positive,
/// finite, non-denormal** `x`.
///
/// This will return unspecified nonsense if `x` is doesn't not
/// satisfy those constraints. Use `log2` if correct handling is
/// required (at the expense of some speed).
///
/// The maximum relative error across all valid input is less than
/// 0.022. The maximum absolute error is less than 0.009.
///
/// |               | Time (ns) |
/// |--------------:|----------------|
/// |    `x.log2()` | 14.3           |
/// |     `log2(x)` | 4.0            |
/// | `log2_raw(x)` | 2.7            |
#[inline]
pub fn log2_raw(x: f32) -> f32 {
    let (_sign, exp, signif) = float::decompose(x);
    debug_assert!(_sign == 0 && 1 <= exp && exp <= 254);

    let high_bit = (signif >> 22) & 1;
    let add_exp = (exp + high_bit) as i32 - 127;
    let normalised = float::recompose(0, 0x7F ^ high_bit, signif) - 1.0;
    const A: f32 = -0.6296735;
    const B: f32 = 1.466967;
    add_exp as f32 + normalised * (B + A * normalised)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck as qc;
    use std::f32 as f;

    #[test]
    fn log2_rel_err_qc() {
        fn prop(x: f32) -> qc::TestResult {
            if !(x > 0.0) { return qc::TestResult::discard() }

            let e = log2(x);
            let t = x.log2();

            if x == 1.0 {
                qc::TestResult::from_bool(e == 0.0)
            } else {
                qc::TestResult::from_bool(((e - t) / t).abs() < 0.025)
            }
        }
        qc::quickcheck(prop as fn(f32) -> qc::TestResult)
    }
    const PREC: u32 = 1 << 20;
    #[test]
    fn log2_rel_err_exhaustive() {
        let mut max = 0.0;
        for i in 0..PREC + 1 {
            for j in -5..6 {
                let x = (1.0 + i as f32 / PREC as f32) * 2f32.powi(j * 20);
                let e = log2(x);
                let t = x.log2();
                let rel = (e - t).abs() / t;
                if rel > max { max = rel }
                assert!((x == 1.0 && e == 0.0) || rel < 0.025,
                        "{:.8}: {:.8}, {:.8}. {:.4}", x, e, t, rel);
            }
        }
        println!("maximum {}", max);
    }

    #[test]
    fn edge_cases() {
        assert!(log2(f::NAN).is_nan());
        assert!(log2(-1.0).is_nan());
        assert!(log2(f::NEG_INFINITY).is_nan());
        assert_eq!(log2(f::INFINITY), f::INFINITY);
        assert_eq!(log2(0.0), f::NEG_INFINITY);
    }

    #[test]
    fn denormals() {
        fn prop(x: u8, y: u16) -> bool {
            let signif = ((x as u32) << 16) | (y as u32);
            let mut x = ::float::recompose(0, 1, signif);

            let mut log = x.log2();
            for _ in 0..23 {
                assert!(x > 0.0);
                let e = log2(x);
                let rel = (e - log).abs() / log;
                if rel >= 0.025 {
                    return false
                }

                x /= 2.0;
                log -= 1.0;
            }
            true
        }
        qc::quickcheck(prop as fn(u8, u16) -> bool)
    }
}
