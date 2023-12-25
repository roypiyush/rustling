use rand::Rng;
use std::io;
use std::io::Write;
use std::process;

fn swap(list: &mut Vec<i32>, i: usize, r: usize) {
    let t = list[i];
    list[i] = list[r];
    list[r] = t;
}

fn partition(list: &mut Vec<i32>, p: i32, r: i32) -> i32 {

    let x = list[r as usize];

    let mut i: i32 = p - 1;
    for j in p..r {
        if list[j as usize] <= x {
            i += 1;
            swap(list, i as usize, j as usize);
        }
    }

    i += 1;
    swap(list, i as usize, r as usize);

    return i;
}

fn quick_sort(list: &mut Vec<i32>, left: i32, right: i32) {

    if left < right {

        let partition = partition(list, left, right);
        quick_sort(list, left, partition - 1);
        quick_sort(list, partition + 1, right);
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

    let size: i32 = size.trim().parse().unwrap_or_else(|_error| {
        println!("Integer was not provided or is not i32");
        process::exit(1);
    });
    

    let mut list_of_numbers: Vec<i32> = Vec::new();
    for _i in 0..size {
        list_of_numbers.push(rand::thread_rng().gen_range(-size..=size));
    }
    println!("Before sorting {:?}", list_of_numbers);

    quick_sort(&mut list_of_numbers, 0, size - 1);
    println!("After sorting {:?}", list_of_numbers);
}
