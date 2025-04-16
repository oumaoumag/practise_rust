#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle{ width, height }
    }
    // Instance method (borrowed self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Instance method (mutable self)
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

// Usage
fn main() {
    // Using associated function
    let mut rect = Rectangle::new(10, 20);

    // Using instance methods
    println!("Area: {}", rect.area());
    rect.resize(15, 25);
    println!("{:?}", rect);
}