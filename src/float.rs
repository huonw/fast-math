use ieee754::Ieee754;

pub const SIGN: usize = 1;
pub const EXP: usize = 8;
pub const SIGNIF: usize = 23;

#[inline]
pub fn flip_sign_nonnan(sign: f32, magnitude: f32) -> f32 {
    let (s1, _, _) = sign.decompose_raw();
    let (s2, e2, m2) = magnitude.decompose_raw();
    f32::recompose_raw(s1 ^ s2, e2, m2)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f32 as f;

    #[test]
    fn test_flip_sign_nonnan() {
        assert_eq!(flip_sign_nonnan(2.0, 3.0), 3.0);
        assert_eq!(flip_sign_nonnan(2.0, -3.0), -3.0);
        assert_eq!(flip_sign_nonnan(-2.0, 3.0), -3.0);
        assert_eq!(flip_sign_nonnan(-2.0, -3.0), 3.0);

        assert_eq!(flip_sign_nonnan(1.0, f::INFINITY), f::INFINITY);
        assert_eq!(flip_sign_nonnan(1.0, f::NEG_INFINITY), f::NEG_INFINITY);
        assert_eq!(flip_sign_nonnan(-1.0, f::INFINITY), f::NEG_INFINITY);
        assert_eq!(flip_sign_nonnan(-1.0, f::NEG_INFINITY), f::INFINITY);
    }
}
