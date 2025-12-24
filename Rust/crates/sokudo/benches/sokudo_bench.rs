use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_universe_sokudo::GBM;

fn benchmark_gbm(c: &mut Criterion) {
    let gbm = GBM::new(0.05, 0.2, 100.0);
    
    // Compare simulating 1000 paths: Serial vs Parallel
    let mut group = c.benchmark_group("GBM Simulation (1000 paths, 100 steps)");
    
    group.bench_function("Sequential", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(gbm.simulate(1.0, 100));
            }
        })
    });
    
    group.bench_function("Parallel", |b| {
        b.iter(|| {
            black_box(gbm.simulate_paths_parallel(1.0, 100, 1000));
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_gbm);
criterion_main!(benches);
