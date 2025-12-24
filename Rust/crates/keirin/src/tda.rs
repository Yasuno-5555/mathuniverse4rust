#[derive(Debug, Clone)]
pub struct PointCloud {
    pub points: Vec<Vec<f64>>,
}

impl PointCloud {
    pub fn new(points: Vec<Vec<f64>>) -> Self {
        Self { points }
    }
    
    // Future TDA methods (persistence, rips complex etc.)
}
