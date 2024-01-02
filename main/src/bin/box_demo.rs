use std::fmt::Display;
struct MyStruct {
    val: i32
}

impl Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "val=({})", self.val)
    }
}

#[test]
fn my_test() {
    let my_struct = MyStruct{
        val: 1
    };
    println!("{}", my_struct);

    let mut boxed_my_struct: Box<MyStruct> = Box::new(my_struct);
    
    println!("{}", boxed_my_struct);
    (*boxed_my_struct).val = 2;
    println!("{}", boxed_my_struct);
}

fn main() {}