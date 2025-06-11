use std::time::Instant;

fn quick_sort(list: &mut Vec<i64>, mut p: usize, mut q: usize) {
    while p < q {
        let partition = partition(list, p, q);

        if partition - p < q - partition {
            if partition > 0 {
                quick_sort(list, p, partition - 1);
            }
            p = partition + 1;
        } else {
            quick_sort(list, partition + 1, q);
            q = partition - 1;
        }
    }
}

fn partition(list: &mut Vec<i64>, p: usize, q: usize) -> usize {
    let pivot = list[q];
    let mut i = p;

    for j in p..q {
        if list[j] < pivot {
            list.swap(i, j);
            i += 1;
        }
    }

    list.swap(i, q);
    i
}

fn main() {
    let mut list = main::get_unsorted_list();

    let instant = Instant::now();

    let p: usize = 0;
    let q: usize = list.len();

    quick_sort(&mut list, p, q - 1);

    println!("Time taken {}", instant.elapsed().as_millis());
    for n in list {
        print!("{n} ");
    }
    println!();
}
