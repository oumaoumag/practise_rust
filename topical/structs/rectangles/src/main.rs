struct Rectangle {
    witdth: u32,
    height: u32,
}
fn main() {
    let witdth1 = 20;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(witdth1, height1)
);

let rect1 = (30, 50);

let rect2 = Rectangle {
    width: 30,
    height: 50,
};

println!(
    "The area of the rectangle is {} square pixels.",
    area3(rect2)
);

println!(
    "The area of the rectangle is {} square pixels.",
    area2(rect1)
);
} 

// Calculates the area of the rectangle
fn area(width: u32, height: u32) -> u32 {
    width + height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32