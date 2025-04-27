use matrix_multiplication::*;

fn main() {
    let matrix = Matrix((1, 3), (4, 5));

    println!("Original Matrix: {:?}", matrix);
    println!("Matrix after multiplication: {:?}", multiply(matrix, 3));
}