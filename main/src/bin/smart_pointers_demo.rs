use std::fmt::Display;

struct MyStruct {
    val: i32,
}

impl Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "val=({})", self.val)
    }
}

#[cfg(test)]
mod test {

    use super::MyStruct;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn box_pointer_demo() {
        let my_struct = MyStruct { val: 1 };
        println!("{}", my_struct);

        let mut boxed_my_struct: Box<MyStruct> = Box::new(my_struct);

        println!("{}", boxed_my_struct);
        (*boxed_my_struct).val = 2;
        println!("{}", boxed_my_struct);
    }

    #[test]
    fn rc_pointer_demo() {
        let my_struct = MyStruct { val: 1 };
        println!("{}", my_struct);

        // creates immuatable references of enclosed value.
        // This means we cannot mutate by Dereferencing
        let my_struct_rc_1: Rc<MyStruct> = Rc::new(my_struct);
        println!("{}", my_struct_rc_1);

        let my_struct_rc_2: Rc<MyStruct> = Rc::clone(&my_struct_rc_1);
        println!("{}", my_struct_rc_2);

    }

    #[test]
    fn refcell_pointer_demo() {
        let my_struct = MyStruct { val: 1 };
        println!("MyStruct: {}", my_struct);

        let my_struct_rc_1 = Rc::new(RefCell::new(my_struct));
        println!("RefCell 1 {}", my_struct_rc_1.borrow());

        // Clone to have multiple references
        let my_struct_rc_2 = Rc::clone(&my_struct_rc_1);

        my_struct_rc_1.borrow_mut().val = 2;
        println!("RefCell 2 {}", my_struct_rc_2.borrow_mut().val);
    }

}

fn main() {}
