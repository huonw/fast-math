extern crate fast_math;


fn main() {
    // literally test all valid normal floats
    let (abs, rel) = ((1 << 23)..(0xFF << 23))
        .map(|i| {
            let x = unsafe {std::mem::transmute(i)};
            let e = fast_math::log2_raw(x);
            let t = x.log2();
            let diff = (e - t).abs();
            (diff, diff / t)
        })
        .fold((0_f32, 0_f32), |(a, a_), (b, b_)| (a.max(b), a_.max(b_)));

    println!("absolute: {:.8}, relative: {:.8}", abs, rel);
}
