use std::str;

fn main() {
    print_method("Hello, world!");
    if_else_demo();
}

fn print_method(string: &str) {
    println!("{}", string);
}

fn if_else_demo() {
    let number = 3;

    if number < 5 {
        println!("if else DEMOed");
    } else {
    }
}