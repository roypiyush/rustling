
enum LinkedList {
    Node(i32, Box<LinkedList>),
    Nil,
}

impl LinkedList {
    fn new() -> LinkedList {
        Nil
    }

    fn prepend(self, value: i32) -> LinkedList {
        Node(value, Box::new(self))
    }
    
    fn len(&self) -> i32 {
        match *self {
            // ref since self is borrowed
            Node(_, ref tail) => 1 + tail.len(),
            Nil => 0 // base case
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Node(head, ref tail) => {
                format!("{} => {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

use crate::LinkedList::{Nil, Node};

fn main() {
    let mut list = LinkedList::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Length = {}\n{}", list.len(), list.stringify());
}
