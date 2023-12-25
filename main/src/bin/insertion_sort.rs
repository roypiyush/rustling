use rand::Rng;
use std::io;
use std::io::Write;
use std::process;


fn insertion_sort(list: &mut Vec<i32>) {
    let size: usize = list.len();
    for i in 1..size {
        let mut j = i;
        while j > 0 {
            if list[j - 1] > list[j] {
                let t = list[j - 1];
                list[j - 1] = list[j];
                list[j] = t;
            }

            j -= 1;
        }
    }
}

fn main() {
    print!("Please provide size : ");
    io::stdout().flush().unwrap();
    let mut size = String::new();

    if let Err(e) = io::stdin().read_line(&mut size) {
        println!("Could not read user input {}", e);
        process::exit(1);
    };

    let size: u8 = size.trim().parse().unwrap_or_else(|_error| {
        println!("Integer was not provided");
        process::exit(1);
    });
    

    let mut list_of_numbers: Vec<i32> = Vec::new();
    for _i in 0..size {
        list_of_numbers.push(rand::thread_rng().gen_range(-10..=10));
    }
    println!("Before sorting {:?}", list_of_numbers);

    insertion_sort(&mut list_of_numbers);
    println!("After sorting {:?}", list_of_numbers);
}
