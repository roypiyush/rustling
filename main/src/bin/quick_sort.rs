use main::get_unsorted_list;
use std::time::Instant;

fn swap(list: &mut Vec<i64>, i: usize, r: usize) {
    (list[i], list[r]) = (list[r], list[i]);
}

fn partition(list: &mut Vec<i64>, p: i64, r: i64) -> i64 {
    let x = list[r as usize];

    let mut i: i64 = p - 1;
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

fn quick_sort(list: &mut Vec<i64>, left: i64, right: i64) {
    if left < right {
        let partition = partition(list, left, right);
        quick_sort(list, left, partition - 1);
        quick_sort(list, partition + 1, right);
    }
}

fn main() {
    let mut list_of_numbers: Vec<i64> = get_unsorted_list();
    let size = list_of_numbers.len();

    let now: Instant = Instant::now();
    quick_sort(&mut list_of_numbers, 0, (size - 1).try_into().unwrap());
    let elapsed_time = now.elapsed();
    println!("Running function() took {} ms", elapsed_time.as_millis());
}
