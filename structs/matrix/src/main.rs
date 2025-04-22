use matrix::*;

fn main() {
    let mut my_matrix = Matrix::new(5, 5);
    println!("Row: {}", my_matrix.rows);
    println!("Cols: {}", my_matrix.cols);
    println!("Data: {:?}", my_matrix.data);


    print_matrix(&my_matrix);

    //Before setting values
    println!("Before Setting Values");

    // Get some Values
    println!("\nGetting values: ");
    if let Some(value) = my_matrix.get(0, 1) {
        println!("Value at (0, 1) is {}", value);
    }

    if let Some(value) = my_matrix.get(2, 4) {
        println!("value at (2, 4 is {}", value)
    }

         print_matrix(&my_matrix);

    
        println!("After Setting Values");

      // Set some values
      println!("\nSet value: ");
      match my_matrix.set(0, 1, 2.5) {
          Ok(_) => println!("Set value at (0, 1)"),
          Err(e) => println!("Error setting value at (0, 1): {}", e),
      }
  
      match my_matrix.set(2, 4, 8.9) {
          Ok(_) => println!("Set value at (2, 4)"),
          Err(e) => println!("Error setting value at (2, 4): {}", e )
      }

    print_matrix(&my_matrix);


}