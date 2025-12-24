use wasm_bindgen::prelude::*;
use math_universe_zigen::Dual;
use math_universe_ryoshi::QuantumState;

#[wasm_bindgen]
pub fn setup() {
    console_error_panic_hook::set_once();
}

// --- Zigen Wrapper ---

#[wasm_bindgen]
pub struct WasmDual {
    inner: Dual<f64>,
}

#[wasm_bindgen]
impl WasmDual {
    #[wasm_bindgen(constructor)]
    pub fn new(real: f64, dual: f64) -> Self {
        Self { inner: Dual::new(real, dual) }
    }
    
    pub fn real(&self) -> f64 { self.inner.real }
    pub fn dual(&self) -> f64 { self.inner.dual }
    
    pub fn add(&self, other: &WasmDual) -> WasmDual {
        WasmDual { inner: self.inner + other.inner }
    }
    
    pub fn mul(&self, other: &WasmDual) -> WasmDual {
        WasmDual { inner: self.inner * other.inner }
    }
    
    pub fn sin(&self) -> WasmDual {
        WasmDual { inner: self.inner.sin() }
    }
}

// --- Ryoshi Wrapper ---

#[wasm_bindgen]
pub struct WasmQuantumState {
    inner: QuantumState,
}

#[wasm_bindgen]
impl WasmQuantumState {
    #[wasm_bindgen(constructor)]
    pub fn new(num_qubits: usize) -> Self {
        Self { inner: QuantumState::new(num_qubits) }
    }
    
    pub fn apply_h(&mut self, target: usize) {
        self.inner.apply_h(target);
    }
    
    pub fn apply_x(&mut self, target: usize) {
        self.inner.apply_x(target);
    }
    
    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        self.inner.apply_cnot(control, target);
    }
    
    /// Returns the probability of measuring 1 at the target qubit.
    pub fn get_probability(&self, target: usize) -> f64 {
        let mut p = 0.0;
        let size = self.inner.amplitudes.len();
        for i in 0..size {
             if (i >> target) & 1 == 1 {
                 p += self.inner.amplitudes[i].norm_sqr();
             }
        }
        p
    }
}
