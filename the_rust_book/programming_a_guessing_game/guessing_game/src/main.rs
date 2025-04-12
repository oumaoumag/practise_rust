use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess A number");
    let secret_number = rand::rng().random_range(1..=100);

  

    loop {
        println!("Enter a number!");

    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        println!("You guessed {}", guess);
    
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        }; 
    
        println!("The secret number is: {secret_number}");


           match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {

            println!("You win");
            break;
            }
        }
    }

}

 