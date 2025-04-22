use matrix::*;

fn main() {
    let my_matrix = Matrix::new(3, 4);
    println!("Row: {}", my_matrix.rows);
    println!("Cols: {}", my_matrix.cols);
    println!("Data: {:?}", my_matrix.data);

    // Get some Values
    println!("\nGetting values: ");
    if let Some(value) = my_matrix.get(0, 1) {
        println!("Value at (0, 1) is {}", value);
    }

    if let Some(value) = my_matrix.get(2, 4) {
        println!("value at (2, 4 is {}", value)
    }
}