extern crate fast_math;


fn main() {
    // literally test all valid normal floats
    let max_abs_error = (0..(1<<23))
        .map(|i| {
            let exp = if i >= (1 << 22) {
                126
            } else {
                127
            };
            let x = fast_math::float::recompose(0, exp, i);
            let e = fast_math::log2_raw(x);
            let t = x.log2();
            (e - t).abs()
        })
        .fold(0.0, |a: f32, b| a.max(b));
    println!("absolute error: {:.8}", max_abs_error);
    let max_rel_error = ((1 << 23)..(0xFF << 23))
        .map(|i| {
            let x = unsafe {std::mem::transmute(i)};
            let e = fast_math::log2_raw(x);
            let t = x.log2();
            (e - t).abs() / t
        })
        .fold(0.0, |a: f32, b| a.max(b));

    println!("relative error: {:.8}", max_rel_error)
}
