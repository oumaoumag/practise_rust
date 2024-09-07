/* Data TYpes 

    We use data types in Rust to determine the type of data associated with the vatiables.
  There re four primary data types in Rust also known as scalar types
    - Interger
    - Floating-Point
    - Boolean
    - Character

     Integer Type 
In Rust, we use interger data types to stre whole numbers. ' let number: i32 == 200;
The integer type i32 has two parts to it:
    - `i` - specifies signed interger type (can store both positive or negative value)
    - `32` - size of the data type (takes 32 bits of space in memory)

    FLoating Point Type
Are used to store fractional numbers(numbers with decimal points)
    - f32
    - f64
Here the `f` character represents a floating number, `32` and `64` represents the size in bits

NOTE: `f32` is a single-precision flaoting type whereas `f64` is double-precision type. With double-precision,
        `f64` can store data with a larger decimal range and is considered more precise.
    
       3. Boolean Type
A boolean data type can have two possible values: `true` or `false`
we use the  `bool` keyword to represt the boolean type in Rust

        4. Character Type
The character type in Rust is used store a character.
we use use use single quotes to represent a character.
we also special characters like $, @, & etc using the special character type
we can also store numbers as characters using single quotes.

    Type Inference
In Rust we can create variables wtihout mentioning a data type.
    let x = 51;
In this case, Rust automatically identifies the data type by looking at the value of the variable `x` and
associates it with the variable. This process is known as Type Inference.
    */ 

    


fn main() {
    // Signed interger type
    let x: i32 = -200;
    let y: i32 = 200;

    println!("x =  {}", x);
    println!("y = {}", y);

    // Unsigned Interger type. Only stores positive integer values. uncoment y below to see.
    let x: u32 = 300;
   // let y: u32 = -4

    println!("x = {}", x);
   // println!("y = {]", y)

   // f32 Floating Point Type
   let x: f32 = 3.1;

   // f64 floating point Type
   let y: f64 = 45.0000031;

   println!("x = {}", x);
   println!("y = {}", y);

   // boolean type
   let flag1: bool = true;
   let flag2: bool = false;

   println!("flag1 = {}", flag1);
   println!("flag2 = {}", flag2);

   // char type
   let character: char = 'z';
   let special_character: char = '$';

   println!("character = {}", character);
   println!("special_character = {}", special_character);
   let numeric_character: char = '5';
   println!("numeric_character = {}", numeric_character);

   let x = 51;

   println!("x = {}", x);
}




