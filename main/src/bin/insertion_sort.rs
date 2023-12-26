use main::get_unsorted_list;
use std::time::Instant;

fn insertion_sort(list: &mut Vec<i64>) {
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
    let mut list_of_numbers: Vec<i64> = get_unsorted_list();

    let now = Instant::now();
    insertion_sort(&mut list_of_numbers);
    let elapsed_time = now.elapsed();
    println!("Running function() took {} ms", elapsed_time.as_millis());
}
