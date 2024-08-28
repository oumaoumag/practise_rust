// 

fn main() {
    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing applyed in changing a string type to int type
    let spaces = "    ";
    println!("Spaces in the first instant {spaces}");
    let spaces = spaces.len();
    println!("spaces.len(), : {spaces}");

    // // mutabilty
    // let mut spaces = "    ";
    // println!{"first mut spaces: {spaces}"};
    // spaces = spaces.len();
    // println!("2nd mut spaces: {spaces}");
    
}