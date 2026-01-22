pub struct Matrix {
    pub values: [u32; 4],
}

impl Matrix {
    pub fn new(values: [u32; 4]) -> Self {
        Self { values }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        Matrix::new([
            self.values[0] * other.values[0] + self.values[1] * other.values[2],
            self.values[0] * other.values[1] + self.values[1] * other.values[3],
            self.values[2] * other.values[0] + self.values[3] * other.values[2],
            self.values[2] * other.values[1] + self.values[3] * other.values[3],
        ])
    }

    pub fn identity() -> Self {
        Self::new([1, 0, 0, 1])
    }
}
