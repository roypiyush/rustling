use core::mem::swap;

fn permute(mut s: [char; 3], mut i: i32, mut j: i32) {

    if i == j {
        println!("{:?}", s);
        return;
    }

    let mut k = i;
    // while k <= j {
    //
    //     swap(c, k, i);
    //
    //
    //     swap(c, k, i);
    //     k = k + 1;
    // }
}

fn main() {
    let s: [char; 3] = ['a', 'b', 'c'];

    permute(s, 0, (s.len() - 1) as i32);
    println!("{:?}", s);
}