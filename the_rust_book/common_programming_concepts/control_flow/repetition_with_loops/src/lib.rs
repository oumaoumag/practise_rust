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

// If you have loops within loops, 'break' and 'continue' apply to the innermost loop at that point.
// You can optionally spcify a 'loop label' on a loop that can then use with 'break' or 'continue' to specify that
// you can use wth 'break' or 'continue' to spcify that those keywords apply to the labled loop instead of the innermost loop.
// Loop labels must begin with a signle quote.
pub fn loop_labels_to_disambiguate_btn_multple_loops() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

pub fn conditional_loops_with_while {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!()
}