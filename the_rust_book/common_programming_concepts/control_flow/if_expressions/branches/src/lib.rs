pub fn check_condition(number: i8) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, and 2");
    }
}

pub fn using_if_in_let_statement() {
    let condition = true; 
    let number = if condition { 5 } else {  6 };

    println!("The value of number is: {number}");
}