// Ultralytics 🚀 AGPL-3.0 License - https://ultralytics.com/license

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use ultralytics_template_rust::add_numbers;

fn benchmark_add_numbers(c: &mut Criterion) {
    c.bench_function("add_numbers", |b| {
        b.iter(|| add_numbers(black_box(100), black_box(200)))
    });
}

criterion_group!(benches, benchmark_add_numbers);
criterion_main!(benches);
