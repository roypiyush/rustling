pub mod parentcomponent; // tells compiler to use 

use parentcomponent::component::MyStructure;
// use garden::vegetables::Asparagus; Also works

fn main() {
    println!("\n{:?}\n", MyStructure{});
}
