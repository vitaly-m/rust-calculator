use criterion::{black_box, criterion_group, criterion_main, Criterion};
use calculator::str_to_evaluable;

pub fn criterion_benchmark(c: &mut Criterion) {
    let e = str_to_evaluable("(6+10-4)/(1+1*2)+1");
    c.bench_function("(6+10-4)/(1+1*2)+1", |b| b.iter(|| black_box(e.eval())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);