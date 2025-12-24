use criterion::{black_box, criterion_group, criterion_main, Criterion};
use math_universe_shinen::MultiVector;
use rand::prelude::*;

fn benchmark_geometric_product(c: &mut Criterion) {
    // Rotating many vectors
    let mut rng = rand::thread_rng();
    let vecs: Vec<MultiVector> = (0..10000).map(|_| {
        MultiVector::new([rng.gen(), rng.gen(), rng.gen(), rng.gen(), rng.gen(), rng.gen(), rng.gen(), rng.gen()])
    }).collect();
    
    let rotor = MultiVector::new([1.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0]); // Simple rotation
    
    c.bench_function("Rotate 10k Vectors", |b| {
        b.iter(|| {
            for v in &vecs {
                black_box(v.rotate(rotor));
            }
        })
    });
}

criterion_group!(benches, benchmark_geometric_product);
criterion_main!(benches);
