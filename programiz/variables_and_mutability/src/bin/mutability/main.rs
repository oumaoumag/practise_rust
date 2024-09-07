// Mutability in Rust

// We use the `mut` keyword before the variable name to create a mutable variable.

fn main() {
    // declare a mutable varaible with value 1
    let mut x = 1;
    println!("value of x = {}", x);

    // change the value og variable x
    x = 2;
    println!("update value of x = {}", x);

    /* We are successfully change the value of `x` variable because we have defined it's initial value 
    using the the `mut` keyword. */

    // Rules for Naming Variables in Rust
    
    /* 1. Rust is a case sensitive language. Hence, lowercase varible and uppercase variables are different
    e.g     `age` is different from `AGE`
             `name` is different from `NAME` */
    
    /* 2. Variables must start with either a letter or an underscore. */
        let age = 31;   // valid and good pracice
        let _age = 31;  //  valid variable
        let 1age = 31;  //  invalid variable

    /* 3. Variable names can only contain letters, digits and underscore character. */
        let age1 = 31;      // valid variable
        let age_num = 31;   //  valid variable
        let s@lary  = 52352;    // Invalid variable

    /* 4. Use underscore if we need to use two words as variable names */
        let first name = "Jack";        // invalid variable
        let first_name = "Jack";        // valid varaible
        .et first_name = "Jack";        // invalid variable
}