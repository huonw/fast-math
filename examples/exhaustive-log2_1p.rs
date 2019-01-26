extern crate fast_math;
extern crate ieee754;
use ieee754::Ieee754;
use std::f64;

fn exact_log2_1p(x: f32) -> f32 {
    ((x as f64).ln_1p() * f64::consts::LOG2_E) as f32
}

fn main() {
    let (abs, rel) = (-1.0).upto(std::f32::MAX)
        .map(|x| {
            let e = fast_math::log2_1p(x);
            let t = exact_log2_1p(x);
            let diff = (e - t).abs();
            (diff, e.rel_error(t).abs())
        })
        .fold((0_f32, 0_f32), |(a, a_), (b, b_)| (a.max(b), a_.max(b_)));

    println!("absolute: {:.8}, relative: {:.8}", abs, rel);
}
