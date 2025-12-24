use rand::prelude::*;
use rand_distr::StandardNormal;
use rayon::prelude::*;

pub struct GBM {
    pub mu: f64,
    pub sigma: f64,
    pub s0: f64,
}

impl GBM {
    pub fn new(mu: f64, sigma: f64, s0: f64) -> Self {
        Self { mu, sigma, s0 }
    }

    pub fn simulate(&self, t: f64, steps: usize) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let dt = t / steps as f64;
        let mut path = Vec::with_capacity(steps + 1);
        path.push(self.s0);
        
        let mut current_s = self.s0;
        for _ in 0..steps {
            let z: f64 = rng.sample(StandardNormal);
            let drift = (self.mu - 0.5 * self.sigma * self.sigma) * dt;
            let diffusion = self.sigma * dt.sqrt() * z;
            current_s *= (drift + diffusion).exp();
            path.push(current_s);
        }
        path
    }

    /// Simulate multiple paths in parallel.
    pub fn simulate_paths_parallel(&self, t: f64, steps: usize, num_paths: usize) -> Vec<Vec<f64>> {
        (0..num_paths)
            .into_par_iter()
            .map(|_| self.simulate(t, steps))
            .collect()
    }
}
