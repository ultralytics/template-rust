<a href="https://www.ultralytics.com/"><img src="https://raw.githubusercontent.com/ultralytics/assets/main/logo/Ultralytics_Logotype_Original.svg" width="320" alt="Ultralytics logo"></a>

# Benchmarks

This directory contains performance benchmarks for the project using [Criterion.rs](https://github.com/bheisler/criterion.rs).

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench example_bench

# Generate detailed report
cargo bench -- --verbose
```

## Viewing Results

Benchmark results are stored in `target/criterion/` and include:

- HTML reports for visualization
- Statistical analysis
- Performance comparison across runs

Open `target/criterion/report/index.html` in your browser to view detailed results.

## Adding New Benchmarks

Create a new file in this directory following the pattern in `example_bench.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ultralytics_template_rust::your_function;

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("your_function", |b| {
        b.iter(|| your_function(black_box(42)))
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

Then register it in `Cargo.toml`:

```toml
[[bench]]
name = "your_bench"
harness = false
```

## Best Practices

- Use `black_box()` to prevent compiler optimizations from skipping code
- Run benchmarks on a quiet system for accurate results
- Compare benchmarks before and after changes
- Document performance characteristics and trade-offs
