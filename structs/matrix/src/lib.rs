#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows:    usize,    // number of rows in the matrix
    pub cols: usize,    // number of columns in the matrix
    pub data: Vec<f64>, // Holds actual matrix data  
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let size = rows * cols;
        let data = vec![0.0; size];
        Matrix { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&f64> {
        if col < self.cols && row < self.rows {
            let index = row * self.cols + col;
            Some(&self.data[index])
        } else {
            None
        }
    }

}
 