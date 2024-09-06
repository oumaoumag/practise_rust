git fn main() {
    // MOdules in Rust help  in splitting a program into logical units for better readability and organization.
    // A module is a collection of items: functions, structs and even other modules

    /*& Defining a Module in Rust */
// The `mod` keyword is used to define a modul. 
    // Syntax of a module
    // mod module_name {
    //     // code
    // }

   
}

 //  a module named config
//  mod config {
//     // a function print inside of the module
//     fn print() {
//         println!("config!");
//     }
//  }

 /* Visibility of Items inside a Module in Rust */

 // Items inside a module can be private or public. By default, a module is private.
 // The `pub` keyword can be used to give a an item  public visibility.
 
 mod config {
    mod config {
        // items in modules by default have private visibility
        fn select() {
            println!("called config::select");
        }
    
        // use the `pub` keyword to override private visibility
        pub fn print() {
            println!("called config::print");
        }
    } 
 }



