extern crate fast_math;
extern crate ieee754;
use ieee754::Ieee754;


fn main() {
    // literally test all valid normal floats
    let (abs, rel) = std::f32::MIN_POSITIVE.upto(std::f32::MAX)
        .map(|x| {
            let e = fast_math::log2_raw(x);
            let t = x.log2();
            let diff = (e - t).abs();
            (diff, e.rel_error(t).abs())
        })
        .fold((0_f32, 0_f32), |(a, a_), (b, b_)| (a.max(b), a_.max(b_)));

    println!("absolute: {:.8}, relative: {:.8}", abs, rel);
}
