mod parentcomponent1; // declares module which is right under current directory

use log::{info, warn, error};
use crate::parentcomponent1::my_struct;
use crate::parentcomponent1::component::my_first_func;


mod parentcomponent2 {
    pub mod component {
        pub mod my_func {
            pub fn print() {
                println!("Print {}", module_path!());
            }
        }
        
    }
}

fn main() {
    //println!("\n{:?}\n", MyStructure {});
    let my_struct = my_struct::MyStructure {};
    println!("\nPrint a different way -> {:?}\n", my_struct);
    my_first_func::print();
    

    parentcomponent2::component::my_func::print();
    parentcomponent1::component::my_second_func::print();

    env_logger::init();

    std::env::set_var("RUST_LOG", "crate=info");

    info!("Info message");
    warn!("Warning message");
    error!("Error message");


}
