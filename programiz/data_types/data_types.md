# Rust Data Types

There are four primary data types in Rust also known as scalar types:

    Integer
    Floating-Point
    Boolean
    Character

## 1. Integer Type

In Rust, we use integer data types to store whole numbers. For example,

let number: i32 = 200;

Here, we have created the number variable of type i32 (integer) and stored the value 200.

The integer type i32 has two parts to it:

    i - specifies signed integer type (can store both positive or negative value)
    32 - size of the data type (takes 32 bits of space in memory)

### Signed Integer Type in Rust

```
fn main() {
    // Signed integer type 
    let x: i32 = -200;
    let y: i32 = 200;

    println!("x = {}", x);
    println!("y = {}", y);
}
```
#### Output:
```
x = -200
y = 200
```

### Unsigned Integer Type

We can also create variables that can only store positive integer values. For example,

```
fn main() {
    // Unsigned integer type
    let x: u32 = 300;

    println!("x = {}", x);
}
```
#### OUtput: 

### Categories of Integer Data Types in Rust

Depending on the size of data, we can further classify the signed and unsigned integer type into various categories:

#### Size	Signed	Unsigned
8-bit	i8  	u8
16-bit	i16 	u16
32-bit	i32 	u32
64-bit	i64 	u64
128-bit	i128	u128

### 2. Rust Floating Point Type

Floating point types are used to store fractional numbers (numbers with decimal points). In Rust, floating-point data types can be divided into:

    f32
    f64
