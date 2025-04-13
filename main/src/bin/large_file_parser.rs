use std::fs;
use std::time::Instant;

fn main() {
    let path = "/mnt/e/large_file.txt";

    // Start timing
    let start = Instant::now();

    // Read the entire file into a string
    let file_contents = fs::read_to_string(path).expect("Failed to read file");

    let mut total_lines = 0;
    let mut total_ints = 0;
    let mut total_sum: i64 = 0;

    // Iterate through each line in the file content
    for line in file_contents.lines() {
        if line.trim().is_empty() {
            continue;
        }

        total_lines += 1;

        // Process comma-separated values in the line
        for num_str in line.split(',') {
            if let Ok(num) = num_str.trim().parse::<i64>() {
                total_sum += num;
                total_ints += 1;
            }
        }
    }

    let duration = start.elapsed();

    // Print summary
    println!("Total lines read: {}", total_lines);
    println!("Total integers: {}", total_ints);
    println!("Sum of all integers: {}", total_sum);
    println!("Time taken: {:.4} seconds", duration.as_secs_f64());
}
