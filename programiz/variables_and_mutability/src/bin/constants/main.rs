// Constants

// A constant is a special type of of variable whose value cannot be changes,
// we use the `const` keyword to create constants in Rust.

fn main(){
    // declare a float constant
    const PI: f32 = 3.14;

    println!("value of PI = {}", PI);

    // Once PI is declared as above is value cannot be changed throught the program

    // Try changing the value of PI
    PI = 535.23;
    println!("Update Value of PI: {}", PI)
    // When we run this code we get an error because PI is a consstant
    
    }