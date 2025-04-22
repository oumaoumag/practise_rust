fn main() {
    /*  Creating a Vector */
    // 1. Using the `vec!` Macro
    let v: Vec <i32> = vec!{1,2,3,4,5};

    // 2. Using the `Vec::new` Method
    let mut v: Vec <i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // 3. Using, the vec! Macro for Empty Vectors:
   // let mut v = vec![];

    /* Accesaing Elements */
    // 1. Indexing:
    let third: i32 = v[2];
    println!("Vector v {third}");

    // Using `get`:
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    /* Modifying Vectors */
    // Adding Elements
    v.push(6);

    // Removing Elements
    let last = v.pop();
    
    // Updating Elements:
    v[1] = 10;

    /* Iterating Over Vectors */
    // Basic `for` Loop:
    for i in &v {
        println!("{}",i);
    }

    // Mutable Iteration

    for i in &mut v {
        *i += 50;
    }


    println!("Hello, world!");
}

