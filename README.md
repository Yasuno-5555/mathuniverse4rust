# MathUniverse4Rust

A high-performance mathematical toolkit for Rust, focusing on Geometric Algebra, Stochastic Simulations, and Linear Algebra.

> [!IMPORTANT]
> **This project is designed, implemented, and maintained by Antigravity (AI).**
> It serves as a demonstration of high-performance Rust code generation and domain-specific library design.

## Features

- **Zigen**: Low-level linear algebra and core primitives.
- **Shinen**: High-performance Geometric Algebra (Clifford Algebra) implementation.
- **Sokudo**: Fast stochastic simulations (e.g., GBM) with built-in parallelization.
- **Risan**: Discrete mathematics and specialized algorithms.
- **Keirin**: Optimization and financial mathematics.
- **Ryoshi**: Quantum computing simulations and complex linear algebra.

## Performance Benchmarks

Executed on a linux environment.

### 1. GBM Simulation (Sokudo)
Simulating 1,000 paths with 100 steps.

| Mode | Mean Time | Speedup |
| :--- | :--- | :--- |
| Sequential | ~4.05 ms | 1.0x |
| **Parallel (Rayon)** | **~1.23 ms** | **~3.3x** |

### 2. Geometric Algebra (Shinen)
Rotating 10,000 vectors in 3D/4D space.

| Task | Mean Time |
| :--- | :--- |
| **10k Vector Rotations** | **~993.20 µs** |

## Crates

All crates are available under the `math_universe_` prefix:
- `math_universe_zigen`
- `math_universe_shinen`
- `math_universe_sokudo`

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
