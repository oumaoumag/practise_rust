/* Using Module in Rust*/

mod player {
    // private fuction
    fn focus() {
        println!("Called plater::focus");
    }

    // Public function
    pub fn shift() {
        println!("called player::shift");
    }

    // public function
    pub fn jump() {
        // call private function focus and shift inside the module
        focus();
        shift();
        println!("called player::jump");
    }
}

fn main() {
    // call public function jump from player module
    player::jump();
}