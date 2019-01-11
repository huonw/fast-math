extern crate fast_math;
extern crate ieee754;
use ieee754::Ieee754;

fn main() {
    let (abs, rel) = (-1_f32).upto(1.0)
        .map(|x| {
            if x.classify() == std::num::FpCategory::Subnormal {
                (0.0, 0.0)
            } else {
                let e = fast_math::atan_raw(x);
                let t = x.atan();
                ((e - t).abs(), e.rel_error(t).abs())
            }
        })
        .fold((0_f32, 0_f32), |(a, a_), (b, b_)| (a.max(b), a_.max(b_)));

    println!("atan_raw: absolute: {:.8}, relative: {:.8}", abs, rel);

    // literally test all valid normal floats
    let max = std::f32::MAX;
    let (abs, rel) = (-max).upto(max)
        .map(|x| {
            if x.classify() == std::num::FpCategory::Subnormal {
                (0.0, 0.0)
            } else {
                let e = fast_math::atan(x);
                let t = x.atan();
                ((e - t).abs(), e.rel_error(t).abs())
            }
        })
        .fold((0_f32, 0_f32), |(a, a_), (b, b_)| (a.max(b), a_.max(b_)));
    println!("atan    : absolute: {:.8}, relative: {:.8}", abs, rel);
}
