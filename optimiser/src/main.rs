use ndarray::prelude::*;
use optimize::vector::{NelderMeadBuilder};
use optimize::scalar::{GoldenRatioBuilder};
use ieee754::Ieee754;
use std::env;

mod problems;

fn max_errors(approx: impl IntoIterator<Item = f32>,
              exact: impl IntoIterator<Item = f64>) -> (f64, f64) {
    let mut rel = 0.0;
    let mut abs = 0.0;
    for (x, y) in approx.into_iter().zip(exact) {
        rel = (x as f64).rel_error(y).abs().max(rel);
        abs = (x as f64 - y).abs().max(abs);
    }
    (rel, abs)
}

trait Approximation {
    /// Name of the function this is approximating
    fn name() -> &'static str;

    /// The number of parameters to optimize
    const NUM_PARAMS: usize;
    /// The ranges (min, max, initial guess) for each parameter.
    fn ranges() -> Vec<(f32, f32, Option<f32>)>;

    /// The minimum value to test relative accuracy
    const MIN: f32;
    /// The maximum value to test relative accuracy
    const MAX: f32;
    /// Any specific values that should be included in the relative
    /// accuracy testing, in case the automatic selection doesn't
    /// include them.
    fn exact_test_values() -> Vec<f32> { vec![] }

    /// The "exact" value of the approximated function at `x`.
    fn exact(x: f64) -> f64;
    /// The value of the approximation at `x` using parameters
    /// `params`.
    fn approx(x: f32, params: ArrayView1<f64>) -> f32;
}

fn run<A: Approximation>(_a: A, num_test_values: usize) {
    let lin_test = Array::linspace(A::MIN, A::MAX, num_test_values);
    let mut test_values = lin_test.to_vec();
    test_values.extend(A::exact_test_values());

    let mut guesses = Array::zeros((A::NUM_PARAMS, 3));

    let ranges = A::ranges();
    assert_eq!(ranges.len(), A::NUM_PARAMS);
    for ((min, max, init), mut row) in ranges.into_iter().zip(guesses.genrows_mut()) {
        row[0] = min as f64;
        row[1] = max as f64;
        row[2] = init.unwrap_or((min + max) / 2.0) as f64;
    }

    if A::NUM_PARAMS == 1 {
        let minimizer = GoldenRatioBuilder::default()
            .xtol(1e-8)
            .max_iter(50000)
            .build()
            .unwrap();

        let func = |point: f64| {
            let slice = &[point];
            let view = ArrayView::from_shape(1, slice).unwrap();
            let approx = test_values.iter().map(|x| A::approx(*x, view));
            let exact = test_values.iter().map(|x| A::exact(*x as f64));

            let (rel, _abs) = max_errors(approx, exact);
            rel
        };
        let result = minimizer.minimize_bracket(&func, guesses[(0, 0)], guesses[(0, 1)]);

        let error = func(result);
        println!("{:10} (rel error = {:.5e}): {:<12}", A::name(), error, result as f32);
    } else {
        let minimizer = NelderMeadBuilder::default()
            .xtol(1e-8)
            .ftol(1e-8)
            .maxiter(50000)
            .build()
            .unwrap();

        let func = |view: ArrayView1<f64>| {
            let approx = test_values.iter().map(|x| A::approx(*x, view));
            let exact = test_values.iter().map(|x| A::exact(*x as f64));

            let (rel, _abs) = max_errors(approx, exact);
            println!("{} {}", view, rel);
            rel
        };
        let result = minimizer.minimize(&func, guesses.column(2));
        let error = func(result.view());
        println!("{:10} (rel error = {:.5e}): {:<12}", A::name(), error,
                 result.map(|x| *x as f32));
    }
}

fn main() {
    let n = 1_000_000;
    for name in env::args().skip(1) {
        match name.as_str() {
            "atan" => run(problems::Atan, n),
            "exp" => run(problems::Exp, n),
            "exp2" => run(problems::Exp2, n),
            "exp_m1" => run(problems::ExpM1, n),
            "log2" => run(problems::Log2, n),
            "log2_1p" => run(problems::Log2_1p, n),
            "log_1p" => run(problems::Log_1p, n),
            s => panic!("unknown argument '{}'", s),
        }
    }
}
