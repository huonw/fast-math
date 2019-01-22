use core::f32;
use core::f32::consts as f;
use float;
use ieee754::Ieee754;

#[derive(Clone, Copy)]
enum Base {
    E,
    Two,
}
impl Base {
    #[inline(always)]
    fn log2(self) -> f32 {
        match self {
            Base::E => f::LOG2_E,
            Base::Two => 1.0,
        }
    }
    #[inline(always)]
    fn ln(self) -> f32 {
        match self {
            Base::E => 1.0,
            Base::Two => f::LN_2,
        }
    }

    #[inline(always)]
    fn upper_limit(self) -> f32 {
        128.0 / self.log2()
    }

    #[inline(always)]
    fn lower_limit(self) -> f32 {
        -127.0 / self.log2()
    }
}

#[inline(always)]
fn exp_raw_impl(x: f32, base: Base) -> f32 {
    const A: f32 = (1 << float::SIGNIF) as f32;
    const MASK: i32 = 0xff800000u32 as i32;
    const EXP2_23: f32 = 1.1920929e-7;
    const C0: f32 = 0.3371894346 * EXP2_23 * EXP2_23;
    const C1: f32 = 0.657636276 * EXP2_23;
    const C2: f32 = 1.00172476;

    let a = A * base.log2();
    let mul = (a * x) as i32;
    let floor = mul & MASK;
    let frac = (mul - floor) as f32;

    let approx = (C0 * frac + C1) * frac + C2;
    f32::from_bits(approx.bits().wrapping_add(floor as u32))
}

#[inline(always)]
fn exp_impl(x: f32, base: Base) -> f32 {
    if x <= base.lower_limit() {
        0.0
    } else if x < base.upper_limit() {
        exp_raw_impl(x, base)
    } else {
        // too big, or NaN, so lets overflow to infinity with some
        // arithmetic to propagate the NaN.
        x + f32::INFINITY
    }
}

const EXP_M1_THRESHOLD: f32 = 0.25153902;
const EXP_M1_ADD: f32 = 1.0053172;
const EXP_M1_MUL: f32 = 0.5004446;
#[inline(always)]
fn exp_m1_raw_impl(x: f32, base: Base) -> f32 {
    if x.abs() <= EXP_M1_THRESHOLD / base.ln() {
        // premultiply because these can be done at compile time
        let add = EXP_M1_ADD * base.ln();
        let mul = EXP_M1_MUL * base.ln() * base.ln();
        x * (add + mul * x)
    } else {
        exp_raw_impl(x, base) - 1.0
    }
}

#[inline(always)]
fn exp_m1_impl(x: f32, base: Base) -> f32 {
    if x.abs() <= EXP_M1_THRESHOLD / base.ln() {
        // premultiply because these can be done at compile time
        let add = EXP_M1_ADD * base.ln();
        let mul = EXP_M1_MUL * base.ln() * base.ln();
        x * (add + mul * x)
    } else {
        exp_impl(x, base) - 1.0
    }
}

/// Compute a fast approximation to 2<sup><code>x</code></sup> for
/// -151 &le; `x` &le; 151.
///
/// This will return unspecified nonsense if `x` does not satisfy
/// those requirements. Use `exp2` if correct handling is required (at
/// the expense of some speed).
///
/// The maximum relative error for inputs for which the result is
/// normal (`x` &ge; -128) is less than 0.011. For `x` < -128, the
/// relative error in the (subnormal) result can be as large as 1.
#[inline]
pub fn exp2_raw(x: f32) -> f32 {
    exp_raw_impl(x, Base::Two)
}

/// Compute a fast approximation to 2<sup><code>x</code></sup>.
///
/// The maximum relative error for inputs for which the result is
/// normal (`x` &ge; -128) is less than 0.011. For `x` < -128, the
/// relative error in the (subnormal) result can be as large as 1.
///
/// If `x` is NaN, `exp2` returns NaN.
///
/// See also `exp2_raw` which only works on -151 &le; `x` &le; 151,
/// but is % faster.
#[inline]
pub fn exp2(x: f32) -> f32 {
    exp_impl(x, Base::Two)
}

/// Compute a fast approximation to 2<sup><code>x</code></sup> - 1 for
/// -128 &le; `x` &le; 128.
///
/// This will return unspecified nonsense if `x` does not satisfy
/// those requirements. Use `exp2_m1` if correct handling is required
/// (at the expense of some speed).
///
/// The maximum relative error is less than 0.011.
#[inline]
pub fn exp2_m1_raw(x: f32) -> f32 {
    exp_m1_raw_impl(x, Base::Two)
}

/// Compute a fast approximation to 2<sup><code>x</code></sup> - 1.
///
/// The maximum relative error is less than 0.011.
///
/// If `x` is NaN, `exp2_m1` returns NaN.
///
/// See also `exp2_m1_raw` which only works on -128 &le; `x` &le; 128,
/// but is 10-50% faster.
#[inline]
pub fn exp2_m1(x: f32) -> f32 {
    exp_m1_impl(x, Base::Two)
}

/// Compute a fast approximation to *e*<sup><code>x</code></sup> for
/// -104 &le; `x` &le; 104.
///
/// This will return unspecified nonsense if `x` does not satisfy
/// those requirements. Use `exp` if correct handling is required (at
/// the expense of some speed).
///
/// The maximum relative error for inputs for which the result is
/// normal (`x` &ge; -128 ln(2) &approx; -88.7) is less than
/// 0.011. For `x` < -128 ln(2), the relative error in the (subnormal)
/// result can be as large as 1.
#[inline]
pub fn exp_raw(x: f32) -> f32 {
    exp_raw_impl(x, Base::E)
}

/// Compute a fast approximation to *e*<sup><code>x</code></sup>.
///
/// The maximum relative error for inputs for which the result is
/// normal (`x` &ge; -128 ln 2 &approx; -88.7) is less than
/// 0.011. For `x` < -128 ln 2, the relative error in the (subnormal)
/// result can be as large as 1.
///
/// If `x` is NaN, `exp` returns NaN.
///
/// See also `exp_raw` which only works on -104 &le; `x` &le; 104,
/// but is % faster.
#[inline]
pub fn exp(x: f32) -> f32 {
    exp_impl(x, Base::E)
}

/// Compute a fast approximation to *e*<sup><code>x</code></sup> - 1 for
/// -88 &le; `x` &le; 88.
///
/// This will return unspecified nonsense if `x` does not satisfy
/// those requirements. Use `exp_m1` if correct handling is required
/// (at the expense of some speed).
///
/// The maximum relative error is less than 0.011.
#[inline]
pub fn exp_m1_raw(x: f32) -> f32 {
    exp_m1_raw_impl(x, Base::E)
}

/// Compute a fast approximation to *e*<sup><code>x</code></sup> - 1.
///
/// The maximum relative error is less than 0.011.
///
/// If `x` is NaN, `exp_m1` returns NaN.
///
/// See also `exp_m1_raw` which only works on -88 &le; `x` &le; 88,
/// but is 10-30% faster.
#[inline]
pub fn exp_m1(x: f32) -> f32 {
    exp_m1_impl(x, Base::E)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{f32, num};

    const PREC: u32 = 1 << 19;

    #[test]
    fn exp_rel_err_exhaustive() {
        let mut max = 0.0;
        for i in 0..PREC + 1 {
            for j in -5..6 {
                for &sign in &[-1.0, 1.0] {
                    let x = sign * (1.0 + i as f32 / PREC as f32) * 2f32.powi(j * 2);
                    let e = exp(x);
                    let t = x.exp();
                    let rel = e.rel_error(t).abs();

                    if t.classify() == num::FpCategory::Subnormal {
                        // subnormal should be approximately right
                        assert!(rel <= 1.0,
                                "{:.8}: e = {:.8e}, t = {:.8e}. {:.4}", x, e, t, rel);
                    } else {
                        if rel > max { max = rel }
                        // e == t handles the infinity case
                        assert!(rel <= 0.002,
                                "{:.8}: e = {:.8e}, t = {:.8e}. {:.4}", x, e, t, rel);
                    }
                }
            }
        }
        println!("maximum {}", max);
    }

    #[test]
    fn exp2_rel_err_exhaustive() {
        let mut max = 0.0;
        for i in 0..PREC + 1 {
            for j in -5..6 {
                for &sign in &[-1.0, 1.0] {
                    let x = sign * (1.0 + i as f32 / PREC as f32) * 2f32.powi(j * 2);
                    let e = exp2(x);
                    let t = x.exp2();
                    let rel = e.rel_error(t).abs();
                    if t.classify() == num::FpCategory::Subnormal {
                        // subnormal should be approximately right
                        assert!(rel <= 1.0,
                                "{:.8}: e = {:.8e}, t = {:.8e}. {:.4}", x, e, t, rel);
                    } else {
                        if rel > max { max = rel }
                        // e == t handles the infinity case
                        assert!(rel <= 0.002,
                                "{:.8}: e = {:.8e}, t = {:.8e}. {:.4}", x, e, t, rel);
                    }
                }
            }
        }
        println!("maximum {}", max);
    }

    #[test]
    fn exp_edge_cases() {
        assert!(exp(f32::NAN).is_nan());
        assert_eq!(exp(f32::NEG_INFINITY), 0.0);
        assert!((exp(0.0) - 1.0).abs() < 0.002);
        assert_eq!(exp(f32::INFINITY), f32::INFINITY);
    }

    #[test]
    fn exp2_edge_cases() {
        assert!(exp2(f32::NAN).is_nan());
        assert_eq!(exp2(f32::NEG_INFINITY), 0.0);
        assert!((exp2(0.0) - 1.0).abs() < 0.002);
        assert_eq!(exp2(f32::INFINITY), f32::INFINITY);
    }

    const EXP_M1_REL_ERR: f32 = 0.0054;
    #[test]
    fn exp_m1_rel_err_exhaustive() {
        let mut max = 0.0;
        for i in 0..PREC + 1 {
            for j in -5..6 {
                for &sign in &[-1.0, 1.0] {
                    let x = sign * (1.0 + i as f32 / PREC as f32) * 2f32.powi(j * 2);
                    let e = exp_m1(x);
                    let t = x.exp_m1();
                    let rel = e.rel_error(t).abs();

                    if t.classify() == num::FpCategory::Subnormal {
                        // subnormal should be approximately right
                        assert!(rel <= 1.0,
                                "{:.8}: e = {:.8e}, t = {:.8e}. {:.4}", x, e, t, rel);
                    } else {
                        if rel > max { max = rel }
                        // e == t handles the infinity case
                        assert!(rel <= EXP_M1_REL_ERR,
                                "{:.8}: e = {:.8e}, t = {:.8e}. {:.4}", x, e, t, rel);
                    }
                }
            }
        }
        println!("maximum {}", max);
    }

    #[test]
    fn exp2_m1_rel_err_exhaustive() {
        let mut max = 0.0;
        for i in 0..PREC + 1 {
            for j in -5..6 {
                for &sign in &[-1.0, 1.0] {
                    let x = sign * (1.0 + i as f32 / PREC as f32) * 2f32.powi(j * 2);
                    let e = exp2_m1(x);
                    let t = (x * f32::consts::LN_2).exp_m1();
                    let rel = e.rel_error(t).abs();
                    if t.classify() == num::FpCategory::Subnormal {
                        // subnormal should be approximately right
                        assert!(rel <= 1.0,
                                "{:.8}: e = {:.8e}, t = {:.8e}. {:.4}", x, e, t, rel);
                    } else {
                        if rel > max { max = rel }
                        // e == t handles the infinity case
                        assert!(rel <= EXP_M1_REL_ERR,
                                "{:.8}: e = {:.8e}, t = {:.8e}. {:.4}", x, e, t, rel);
                    }
                }
            }
        }
        println!("maximum {}", max);
    }

    #[test]
    fn exp_m1_edge_cases() {
        assert!(exp_m1(f32::NAN).is_nan());
        assert_eq!(exp_m1(f32::NEG_INFINITY), -1.0);
        assert_eq!(exp_m1(0.0), 0.0);
        assert_eq!(exp_m1(f32::INFINITY), f32::INFINITY);
    }

    #[test]
    fn exp2_m1_edge_cases() {
        assert!(exp2_m1(f32::NAN).is_nan());
        assert_eq!(exp2_m1(f32::NEG_INFINITY), -1.0);
        assert_eq!(exp2_m1(0.0), 0.0);
        assert_eq!(exp2_m1(f32::INFINITY), f32::INFINITY);
    }
}
