use rand::Rng;
use std::io;
use std::io::Write;
use std::process;

pub fn get_unsorted_list() -> Vec<i64> {
    print!("Please provide size : ");
    io::stdout().flush().unwrap();

    let mut size = String::new();

    if let Err(e) = io::stdin().read_line(&mut size) {
        println!("Could not read user input {}", e);
        process::exit(1);
    };

    let size: usize = size.trim().parse().unwrap_or_else(|_error| {
        println!("Integer was not provided or is not i64");
        process::exit(1);
    });

    unsorted_list(size)
}

pub fn unsorted_list(size: usize) -> Vec<i64> {
    let mut list_of_numbers: Vec<i64> = Vec::new();
    for _i in 0..size {
        list_of_numbers.push(rand::thread_rng().gen_range(-(size as i64)..=(size as i64)));
    }

    list_of_numbers
}
