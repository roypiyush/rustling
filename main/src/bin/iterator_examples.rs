fn squared() {
    let squared = String::new();

    let array = [1, 2, 3, 4, 5];

    let mut fold = array.iter().fold(squared, |acc, x| format!("{acc} + {x}²"));

    fold.replace_range(0..3, "");

    print!("{fold} = ");

    let collect: Vec<i32> = array.iter().map(|x| x * x).collect();

    println!("{}", collect.iter().sum::<i32>());
}

fn main() {
    squared();
}
