fn main() {
    
    let mut guess = String::new();

    // Add text to the end
    guess.push_str("Hello ");

    // Adding a single character
    guess.push('!');

    // Clear the string
    // guess.clear();

    // Replacing the entire contents
    guess = String::from("Godwin is the name");
    println!("After String::from(): {}\n", guess);
        
    // Using format! macro to create new content
    guess = format!("number {}\n", 42);
    

    println!("After format: {}", guess);
}