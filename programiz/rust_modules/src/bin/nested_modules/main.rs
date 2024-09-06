// A module can be defined inside another module,
// This is known as module nesting

/* Note: 
        The `::` operator is used to separate the module name and the item to call inside the module*/ 

/* The Use KeyWord in Rust */

/* we can use the `use` keyword to bring items inside a module into the current scope.
 It helps us eliminate writing out the full module path to call functions. */

// bring the create function into scope
use player::sprite::create;

pub mod player {
    pub mod sprite {
        pub fn create() {
            println!("called player::sprite::create")
        }
    }
}

fn main() {
    // call public function create from sprite module which is inside player module
    create();
}