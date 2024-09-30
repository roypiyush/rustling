use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("a"));
    map.insert(2, String::from("b"));
    println!("{:?}", map);
    map.insert(3, String::from("c"));
    map.insert(2, String::from("d"));

    println!("{:?}", map);

    let width = 4;
    let height = 4;

    let mut array = vec![vec![0; width]; height];
    array[2][2] = 5;

    println!("{:?}", array);

    const WIDTH: usize = 4;
    const HEIGHT: usize = 4;

    let mut array = [[0 as u8; WIDTH]; HEIGHT];
    array[2][2] = 5;

    println!("{:?}", array);
}
