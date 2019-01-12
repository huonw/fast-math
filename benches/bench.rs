extern crate fast_math;

#[macro_use]
extern crate criterion;
use criterion::{Criterion, Fun, black_box};

fn bench<Fast, Raw, Std>(c: &mut Criterion, name: &str, values: &'static [f32],
                         fast: &'static Fast, raw: &'static Raw, std: &'static Std)
where
    Fast: 'static + Fn(f32) -> f32,
    Raw: 'static + Fn(f32) -> f32,
    Std: 'static + Fn(f32) -> f32,
{
    let scalar_baseline = Fun::new(
        "baseline",
        |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(*x); })
        });

    let scalar_full = Fun::new(
        "full",
        move |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(fast(*x)); } )
        });
    let scalar_raw = Fun::new(
        "raw",
        move |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(raw(*x)); } )
        });
    let scalar_std = Fun::new(
        "std",
        move |b, i: &&[f32]| {
            b.iter(|| for x in *i { black_box(std(*x)); } )
        });

    c.bench_functions(&format!("scalar/{}", name),
                      vec![scalar_baseline, scalar_full, scalar_raw, scalar_std],
                      values);

    let vector_baseline = Fun::new(
        "baseline",
        |b, i: &&[f32]| {
            let mut out = vec![0.0; i.len()];
            b.iter(|| {
                for (x, y) in i.iter().zip(&mut out) {
                    *y = *x
                }
                black_box(&out);
            })
        });

    let vector_full = Fun::new(
        "full",
        move |b, i: &&[f32]| {
            let mut out = vec![0.0; i.len()];
            b.iter(|| {
                for (x, y) in i.iter().zip(&mut out) {
                    *y = fast(*x)
                }
                black_box(&out);
            })
        });
    let vector_raw = Fun::new(
        "raw",
        move |b, i: &&[f32]| {
            let mut out = vec![0.0; i.len()];
            b.iter(|| {
                for (x, y) in i.iter().zip(&mut out) {
                    *y = raw(*x)
                }
                black_box(&out);
            })
        });
    let vector_std = Fun::new(
        "std",
        move |b, i: &&[f32]| {
            let mut out = vec![0.0; i.len()];
            b.iter(|| {
                for (x, y) in i.iter().zip(&mut out) {
                    *y = std(*x)
                }
                black_box(&out);
            })
        });

    c.bench_functions(&format!("vector/{}", name),
                      vec![vector_baseline, vector_full, vector_raw, vector_std],
                      values);
}


fn bench_log2(c: &mut Criterion) {
    let values = &[
        0.85708036,  2.43390621,  2.80163358,  2.55126348,  3.18046186,
        2.88689427,  0.32215155,  0.07701401,  1.22922506,  0.4580259 ,
        0.01257442,  4.23107197,  0.89538113,  1.65219582,  0.14632742,
        1.68663984,  1.88125115,  2.16773942,  1.27461936,  1.03091265
    ];
    bench(c, "log2", values, &fast_math::log2, &fast_math::log2_raw, &f32::log2)
}

fn bench_atan(c: &mut Criterion) {
    let values = &[
        0.85708036,  -2.43390621,  2.80163358,  -2.55126348,  3.18046186,
        -2.88689427,  0.32215155,  -0.07701401,  1.22922506,  -0.4580259,
        0.01257442,  -4.23107197,  0.89538113,  -1.65219582,  0.14632742,
        -1.68663984,  1.88125115,  -2.16773942,  1.27461936,  -1.03091265
    ];
    bench(c, "atan", values, &fast_math::atan, &fast_math::atan_raw, &f32::atan)
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
    c.bench_functions("scalar/atan2", vec![baseline, full, std], values);
}

criterion_group!(benches, bench_log2, bench_atan, bench_atan2);
criterion_main!(benches);
