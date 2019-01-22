use ndarray::prelude::*;
use crate::Approximation;
use ieee754::Ieee754;
use std::f32::{self, consts};

pub struct Exp;
impl Approximation for Exp {
    fn name() -> &'static str { "exp" }

    const NUM_PARAMS: usize = 3;
    fn ranges() -> Vec<(f32, f32, Option<f32>)> {
        vec![(0.0, 3.0, Some(0.3371894346)),
             (0.0, 3.0, Some(0.657636276)),
             (0.0, 3.0, Some(1.00172476))]
    }

    const MIN: f32 = -87.0;
    const MAX: f32 = 87.0;
    fn exact_test_values() -> Vec<f32> {
        vec![0.0, 1.0]
    }

    fn exact(x: f64) -> f64 {
        x.exp()
    }

    fn approx(x: f32, params: ArrayView1<f64>) -> f32 {
        assert_eq!(params.len(), Self::NUM_PARAMS);

        const A: f32 = (1 << 23) as f32;
        const MASK: i32 = 0xff800000u32 as i32;
        const EXP2_23: f32 = 1.1920929e-7;
        let c0: f32 = params[0] as f32 * EXP2_23 * EXP2_23;
        let c1: f32 = params[1] as f32 * EXP2_23;
        let c2: f32 = params[2] as f32;

        let a = A * consts::LOG2_E;
        let mul = (a * x) as i32;
        let floor = mul & MASK;
        let frac = (mul - floor) as f32;

        let approx = (c0 * frac + c1) * frac + c2;
        f32::from_bits(approx.bits().wrapping_add(floor as u32))
    }
}

pub struct Exp2;
impl Approximation for Exp2 {
    fn name() -> &'static str { "exp2" }

    const NUM_PARAMS: usize = 3;
    fn ranges() -> Vec<(f32, f32, Option<f32>)> {
        vec![(0.0, 3.0, Some(0.3371894346)),
             (0.0, 3.0, Some(0.657636276)),
             (0.0, 3.0, Some(1.00172476))]
    }

    const MIN: f32 = -87.0;
    const MAX: f32 = 87.0;
    fn exact_test_values() -> Vec<f32> {
        vec![0.0, 1.0]
    }

    fn exact(x: f64) -> f64 {
        x.exp2()
    }

    fn approx(x: f32, params: ArrayView1<f64>) -> f32 {
        assert_eq!(params.len(), Self::NUM_PARAMS);

        const A: f32 = (1 << 23) as f32;
        const MASK: i32 = 0xff800000u32 as i32;
        const EXP2_23: f32 = 1.1920929e-7;
        let c0: f32 = params[0] as f32 * EXP2_23 * EXP2_23;
        let c1: f32 = params[1] as f32 * EXP2_23;
        let c2: f32 = params[2] as f32;

        let a = A;
        let mul = (a * x) as i32;
        let floor = mul & MASK;
        let frac = (mul - floor) as f32;

        let approx = (c0 * frac + c1) * frac + c2;
        f32::from_bits(approx.bits().wrapping_add(floor as u32))
    }
}

pub struct Log2;
impl Approximation for Log2 {
    fn name() -> &'static str { "log2" }

    const NUM_PARAMS: usize = 2;
    fn ranges() -> Vec<(f32, f32, Option<f32>)> {
        vec![(-3.0, 0.0, Some(-0.6296735)),
             (0.0, 3.0, Some(1.466967))]
    }

    const MIN: f32 = f32::MIN_POSITIVE;
    const MAX: f32 = 50.0;

    fn exact_test_values() -> Vec<f32> {
        vec![1.0, 2.0]
    }

    fn exact(x: f64) -> f64 {
        x.log2()
    }

    fn approx(x: f32, params: ArrayView1<f64>) -> f32 {
        assert_eq!(params.len(), Self::NUM_PARAMS);

        let (_sign, exp, signif) = x.decompose_raw();
        debug_assert!(!_sign && 1 <= exp && exp <= 254);

        let high_bit = ((signif >> 22) & 1) as u8;
        let add_exp = (exp + high_bit) as i32 - 127;
        let normalised = f32::recompose_raw(false, 0x7F ^ high_bit, signif) - 1.0;
        let a: f32 = params[0] as f32;
        let b: f32 = params[1] as f32;
        add_exp as f32 + normalised * (b + a * normalised)
    }
}

pub struct Atan;
impl Approximation for Atan {
    fn name() -> &'static str { "atan" }

    const NUM_PARAMS: usize = 2;
    fn ranges() -> Vec<(f32, f32, Option<f32>)> {
        let n2 = 0.273;
        vec![(0.0, 2.0, Some(consts::FRAC_PI_4 + n2)),
             (0.0, 2.0, Some(n2))]
    }

    const MIN: f32 = -1.0;
    const MAX: f32 = 1.0;
    fn exact_test_values() -> Vec<f32> {
        vec![-1.0, 0.0, 1.0]
    }

    fn exact(x: f64) -> f64 {
        x.atan()
    }

    fn approx(x: f32, params: ArrayView1<f64>) -> f32 {
        assert_eq!(params.len(), Self::NUM_PARAMS);

        let add = params[0] as f32;
        let mul = params[1] as f32;

        (add - mul * x.abs()) * x
    }
}
