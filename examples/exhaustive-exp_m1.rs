extern crate fast_math;
extern crate ieee754;
use ieee754::Ieee754;

use std::f32::consts::LN_2;

fn main() {
    let mut max_rel = 0f32;
    let mut max_abs = 0f32;
    for x in (-128.0 * LN_2).upto(128.0 * LN_2) {
        let e = fast_math::exp_m1(x);
        let t = x.exp_m1();
        let diff = (e - t).abs();
        let rel = e.rel_error(t).abs();
        max_abs = max_abs.max(diff);
        max_rel = max_rel.max(rel);
    }
    println!("exp_m1 : absolute: {:.8e}, relative: {:.8}", max_abs, max_rel);

    let (abs, rel) = (-128.0).upto(128.0)
        .map(|x| {
            let e = fast_math::exp2_m1(x);
            let t = (x * LN_2).exp_m1();
            let diff = (e - t).abs();
            let rel = e.rel_error(t).abs();
            (diff, rel)
        })
        .fold((0_f32, 0_f32), |(a, a_), (b, b_)| (a.max(b), a_.max(b_)));

    println!("exp2_m1: absolute: {:.8e}, relative: {:.8}", abs, rel);

}
