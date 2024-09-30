fn main() {
    for i in 0..100000 {
        print!("{}={}  ", i, std::char::from_u32(i).unwrap_or('?'));
    }
    println!();
}
