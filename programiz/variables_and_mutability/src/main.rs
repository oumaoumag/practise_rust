/*  Rust Variables and Mutability */

// In computer programming,  we use variables to store data.
// we can think of variables as containers that hold information.

fn main() {
    /* Variable Declaration */

    // variable to store integer value
    let age = 31;
    println!("Age = {}",age);

    // variables to store floating-point value(decimal point values)
    let salary = 342523.23;
    println!("Salary: {}", salary);

    // Variable to store a string
    let name = "Jackie";
    println!("Name: {}", name);

    /* Change Value of a Variable */
/* By default, Rust variables are immutabl, meaning we cannot change the value of a variable once it is defined. */

// declare a variable with value 1
    let x = 1;
    println!("x = {}", x);

    // change the value of variable x
    x = 2;
    println!("x = {}", x);
    /* running the above code give an error because we are trying to change the value of the `x` variable
        from  `1` to `2` */

}
