pub fn repeating_code_with_loop() {
    loop {
        println!("again!");
    }
}

pub fn returning_values_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result  is {result}");
}