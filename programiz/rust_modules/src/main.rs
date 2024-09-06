git fn main() {
    // MOdules in Rust help  in splitting a program into logical units for better readability and organization.
    // A module is a collection of items: functions, structs and even other modules

    /*& Defining a Module in Rust */
// The `mod` keyword is used to define a modul. 
    // Syntax of a module
    mod module_name {
        // code
    }

   
}

 //  a module named config
 mod config {
    // a function print inside of the module
    fn print() {
        println!("config!");
    }
 }

 