use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("a"));
    map.insert(2, String::from("b"));
    println!("{:?}", map);
    map.insert(3, String::from("c"));
    map.insert(2, String::from("d"));

    println!("{:?}", map);
}
