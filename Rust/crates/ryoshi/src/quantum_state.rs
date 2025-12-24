use num_complex::Complex64;
use std::f64::consts::FRAC_1_SQRT_2;

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumState {
    pub amplitudes: Vec<Complex64>,
    pub num_qubits: usize,
}

impl QuantumState {
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut amplitudes = vec![Complex64::new(0.0, 0.0); size];
        amplitudes[0] = Complex64::new(1.0, 0.0); // |0...0> state
        Self {
            amplitudes,
            num_qubits,
        }
    }

    pub fn apply_x(&mut self, target: usize) {
        if target >= self.num_qubits { return; }
        
        let size = self.amplitudes.len();
        // bit flip pair indices
        // for each basis state, if bit is 0, swap with bit 1
        // Iterate only half states to avoid double swap
        for i in 0..size {
             if (i >> target) & 1 == 0 {
                 let j = i | (1 << target);
                 // Swap amplitudes i and j
                 self.amplitudes.swap(i, j);
             }
        }
    }

    pub fn apply_h(&mut self, target: usize) {
        if target >= self.num_qubits { return; }
        
        let size = self.amplitudes.len();
        // H = 1/sqrt(2) * [[1, 1], [1, -1]]
        // |0> -> (|0> + |1>) / sqrt(2)
        // |1> -> (|0> - |1>) / sqrt(2)
        
        // We need to iterate over pairs (i, j) where i has 0 at target, j has 1 at target
        for i in 0..size {
            if (i >> target) & 1 == 0 {
                let j = i | (1 << target);
                let a = self.amplitudes[i];
                let b = self.amplitudes[j];
                
                self.amplitudes[i] = (a + b) * FRAC_1_SQRT_2;
                self.amplitudes[j] = (a - b) * FRAC_1_SQRT_2;
            }
        }
    }

    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        if control >= self.num_qubits || target >= self.num_qubits || control == target { return; }
        
        let size = self.amplitudes.len();
        for i in 0..size {
            // If control bit is set (1)
            if (i >> control) & 1 == 1 {
                // Determine pair to swap based on target
                if (i >> target) & 1 == 0 {
                    let j = i | (1 << target);
                    self.amplitudes.swap(i, j);
                }
            }
        }
    }
}
