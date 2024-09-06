// A module can be defined inside another module,
// This is known as module nesting

/* Note: 
        The `::` operator is used to separate the module name and the item to call inside the module*/ 

pub mod player {
    pub mod sprite {
        pub fn create() {
            println!("called player::sprite::create")
        }
    }
}

fn main() {
    // call public function create from sprite module which is inside player module
    player::sprite::create();
}