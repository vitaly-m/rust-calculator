use std::str::FromStr;

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use calculator::{Evaluable, Operator, OperatorResult};

pub fn criterion_benchmark(c: &mut Criterion) {
    let e = <Evaluable<OperatorResult>>::from_str("(6+10-4)/(1+1*2)+1").unwrap();
    c.bench_function("(6+10-4)/(1+1*2)+1", |b| b.iter(|| {
        if let OperatorResult::F64(v) = e.eval() {}
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);