// Rust Print Output

// In rust we use the print! or println! macro to print strings, numbers and 
// variables on the output screen.

/*  Point: 
        A `macro in Rust is a piece of code that generates another piece of code
        */

fn main() {
    // THe `print!` macro  prints the text inside double quotes.
    print!("Hello, Godwin! ");
    print!("Rust is fun! I love Rust programming.");
    // `println!` macro adds a newline character(enter) at the end, 
    println!("Hello, world!");

    /* Print Variables*/

    let age = 31;

    // print the variable using println!
    println!("{}", age);

    // print the variable using print!
    print!("{}", age);

    /* `{}` is a placeholder which is replaced by the value of the variabe after the comma.
        we can  also add text with the placeholder to format our output */

    let _new_age = 32;

    // print the variable using println!
    println!("New_Age =  {}", age);

    /* Print Multiple Variables */
    // we can use a single `println!` macro to print multiple variable together.

    let _age_3 = 33;
    let name = "jack";

    // print the variables using println!
    println!("Name = {}, Age = {}", name, _age_3);

    // We can also specify the numbering  (indexing) for placeholders to print varibles
    println!("Name = {1}, Age = {0}", _age_3, name);

    // Similarly, we can also use the variable names directly inside the placeholder.
    println!("Name = {name}, Age = {age}");

    /* Print Newline Character */
    // We can print newline character(s) using the \n escape sequence.
    print!("Rust is fun!\nI love Rust programming.\n");

}
