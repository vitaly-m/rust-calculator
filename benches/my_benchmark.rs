use std::str::FromStr;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use calculator::{Evaluable, EvaluableResult, Expression};

pub fn criterion_benchmark(c: &mut Criterion) {
    let e = <Expression<EvaluableResult>>::from_str("(6+10-4)/(1+1*2)+1").unwrap();
    c.bench_function("(6+10-4)/(1+1*2)+1", |b| {
        b.iter(|| if let EvaluableResult::F64(v) = e.eval() {})
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
