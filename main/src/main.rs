pub mod parentcomponent; // tells compiler to use

use crate::parentcomponent::component::MyStructure;

fn main() {
    println!("\n{:?}\n", MyStructure {});
}
