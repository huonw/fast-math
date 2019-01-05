use core::mem;

const SIGN: usize = 1;
const EXP: usize = 8;
const SIGNIF: usize = 23;

#[inline(always)]
pub fn abs(x: f32) -> f32 {
    let (_, e, m) = decompose(x);
    recompose(0, e, m)
}

/// The sign of `x`, +/-1.0 when `x` is not NaN (NB. this explicitly
/// includes `x == 0`!), and NaN when `x` is NaN.
#[inline]
pub fn signum(x: f32) -> f32 {
    let (s, _, _) = decompose(x);
    let (_, e1, m1) = decompose(1.0);
    let newbits = recompose(s, e1, m1);

    if x.is_nan() {
        x
    } else {
        newbits
    }
}

#[inline(always)]
pub fn decompose(x: f32) -> (u32, u32, u32) {
    let bits: u32 = unsafe { mem::transmute(x) };

    macro_rules! mask{
        ($current: expr => $($other: expr),*) => {
            (bits >> (0 $(+ $other)*)) & ((1 << $current) - 1)
        }
    }

    (mask!(SIGN => EXP, SIGNIF),
     mask!(EXP => SIGNIF),
     mask!(SIGNIF =>))
}

#[inline(always)]
pub fn recompose(sign: u32, exp: u32, signif: u32) -> f32 {
    debug_assert!(sign <= 1);
    debug_assert!(exp <= 255);
    debug_assert!(signif < 1 << 24);

    macro_rules! unmask {
        ($x: expr => $($other: expr),*) => {
            $x << (0 $(+ $other)*)
        }
    }
    let bits =
        unmask!(sign => EXP, SIGNIF) |
        unmask!(exp => SIGNIF) |
        unmask!(signif => );

    unsafe {mem::transmute(bits)}
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck as qc;
    use std::f32 as f;

    #[test]
    fn test_abs() {
        assert_eq!(abs(0.0), 0.0);
        assert_eq!(abs(1.0), 1.0);
        assert_eq!(abs(-1.0), 1.0);
        assert_eq!(abs(12.34), 12.34);
        assert_eq!(abs(-12.34), 12.34);
        assert_eq!(abs(f::INFINITY), f::INFINITY);
        assert_eq!(abs(-f::INFINITY), f::INFINITY);
        assert!(abs(f::NAN).is_nan());
    }

    #[test]
    fn test_signum() {
        assert_eq!(signum(0.0), 1.0);
        assert_eq!(signum(-0.0), -1.0);
        assert_eq!(signum(1.0), 1.0);
        assert_eq!(signum(-1.0), -1.0);
        assert_eq!(signum(12.34), 1.0);
        assert_eq!(signum(-12.34), -1.0);
        assert_eq!(signum(f::INFINITY), 1.0);
        assert_eq!(signum(-f::INFINITY), -1.0);
        assert!(signum(f::NAN).is_nan());
    }

    #[test]
    fn round_trip() {
        fn prop(x: f32) -> qc::TestResult {
            if x.is_nan() { return qc::TestResult::discard() }

            let (sign, exp, signif) = decompose(x);
            let y = recompose(sign, exp, signif);
            qc::TestResult::from_bool(x == y)
        }
        qc::quickcheck(prop as fn(f32) -> qc::TestResult)
    }

    #[test]
    fn smoke() {
        assert_eq!(decompose(0.0), (0, 0, 0));

        assert_eq!(decompose(1.0), (0, 127, 0));
        assert_eq!(decompose(-1.0), (1, 127, 0));

        assert_eq!(decompose(0.5), (0, 126, 0));
        assert_eq!(decompose(-2.0), (1, 128, 0));

        assert_eq!(decompose(1.25), (0, 127, 0b010_0000_0000_0000_0000_0000));
        assert_eq!(decompose(-(2048.0 + 1024.0 + 1.0/4096.0)),
                   (1, 127 + 11, 0b100_0000_0000_0000_0000_0001));

    }
}
