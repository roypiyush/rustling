use main::get_unsorted_list;
use std::time::Instant;

fn main() {
    let mut list_of_numbers: Vec<i64> = get_unsorted_list();
    let size = list_of_numbers.len();

    let now = Instant::now();
    merge_sort(&mut list_of_numbers, 0, size - 1);
    let elapsed_time = now.elapsed();
    println!("Running function() took {} ms", elapsed_time.as_millis());
}

fn merge(list: &mut Vec<i64>, p: usize, m: usize, r: usize) {
    if p > m || m > r {
        return;
    }

    let mut l_ptr = p;
    let mut r_ptr = m + 1;

    let mut copy: Vec<i64> = Vec::new();
    while l_ptr <= m || r_ptr <= r {
        if l_ptr <= m && r_ptr <= r && list[l_ptr] <= list[r_ptr] {
            copy.push(list[l_ptr]);
            l_ptr += 1;
        } else if l_ptr <= m && r_ptr <= r && list[l_ptr] > list[r_ptr] {
            copy.push(list[r_ptr]);
            r_ptr += 1;
        } else if l_ptr <= m {
            copy.push(list[l_ptr]);
            l_ptr += 1;
        } else {
            copy.push(list[r_ptr]);
            r_ptr += 1;
        }
    }

    let mut ptr = p;
    for i in &copy {
        list[ptr] = *i;
        ptr += 1;
    }
}

fn merge_sort(list: &mut Vec<i64>, left: usize, right: usize) {
    if left < right {
        let mid = left + (right - left) / 2;
        merge_sort(list, left, mid);
        merge_sort(list, mid + 1, right);
        merge(list, left, mid, right);
    }
}
