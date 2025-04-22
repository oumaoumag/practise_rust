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

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), String> {
        if col < self.cols && row < self.rows {
            let index = row * self.cols + col;
            self.data[index] = value;
            Ok(())
        } else {
            Err("Error out bounds".to_string())
        }
    }

}
 

pub fn print_matrix(matrix: &Matrix) {
    for r in 0..matrix.rows {
        for c in 0..matrix.cols {
            if let Some(value) = matrix.get(r, c) {
                print!("{} ", value);
            }
        }
        println!();
    }
}