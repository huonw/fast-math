extern crate fast_math;

#[macro_use]
extern crate criterion;
use criterion::{Criterion, Fun, black_box};

fn bench_log2(c: &mut Criterion) {
    let baseline = Fun::new(
        "baseline",
        |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(*x); })
        });

    let full = Fun::new(
        "full",
        |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(fast_math::log2(*x)); } )
        });
    let raw = Fun::new(
        "raw",
        |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(fast_math::log2_raw(*x)); } )
        });
    let std = Fun::new(
        "std",
        |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(x.log2()); } )
        });

    let values = &[
        0.85708036,  2.43390621,  2.80163358,  2.55126348,  3.18046186,
        2.88689427,  0.32215155,  0.07701401,  1.22922506,  0.4580259 ,
        0.01257442,  4.23107197,  0.89538113,  1.65219582,  0.14632742,
        1.68663984,  1.88125115,  2.16773942,  1.27461936,  1.03091265
    ];
    c.bench_functions("log2", vec![baseline, full, raw, std], values);
}

fn bench_atan(c: &mut Criterion) {
    let baseline = Fun::new(
        "baseline",
        |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(*x); })
        });

    let full = Fun::new(
        "full",
        |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(fast_math::atan(*x)); } )
        });
    let raw = Fun::new(
        "raw",
        |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(fast_math::atan_raw(*x)); } )
        });
    let std = Fun::new(
        "std",
        |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(x.atan()); } )
        });

    let values = &[
        0.85708036,  -2.43390621,  2.80163358,  -2.55126348,  3.18046186,
        -2.88689427,  0.32215155,  -0.07701401,  1.22922506,  -0.4580259,
        0.01257442,  -4.23107197,  0.89538113,  -1.65219582,  0.14632742,
        -1.68663984,  1.88125115,  -2.16773942,  1.27461936,  -1.03091265
    ];
    c.bench_functions("atan", vec![baseline, full, raw, std], values);
}

fn bench_atan2(c: &mut Criterion) {
    let baseline = Fun::new(
        "baseline",
        |b, i: &&[(f32, f32)]| {
            b.iter(|| for &(x, y) in *i { black_box((x, y)); })
        });

    let full = Fun::new(
        "full",
        |b, i: &&[(f32, f32)]| {
            b.iter(|| for &(x, y) in *i { black_box(fast_math::atan2(x, y)); } )
        });
    let std = Fun::new(
        "std",
        |b, i: &&[(f32, f32)]| {
            b.iter(|| for &(x, y) in *i { black_box(x.atan2(y)); } )
        });


    let values = &[
        (0.85708036,  2.43390621), (2.80163358,  -2.55126348),
        (-3.18046186, -2.88689427), (-0.32215155,  0.07701401),
        (1.22922506,  0.4580259), (0.01257442,  -4.23107197),
        (-0.89538113,  -1.65219582), (-0.14632742, 1.68663984),
        (1.88125115,  2.16773942), (1.27461936,  -1.03091265)
    ];
    c.bench_functions("atan2", vec![baseline, full, std], values);
}

criterion_group!(benches, bench_log2, bench_atan, bench_atan2);
criterion_main!(benches);
