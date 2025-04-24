// use std::collections::HashMap;
use counting_words::*;

fn main() {
    println!("{:?}", counting_words("Hello, world!"));
    println!("{:?}", counting_words("\"Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.\"
    ― Albert Einstein "));
    println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
}