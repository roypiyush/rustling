

struct LinkedList<T> {
    val: Option<T>,
    next: Option<Box<LinkedList<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            val: None,
            next: None,
        }
    }

    fn prepend(self, val: T) -> LinkedList<T> {
        LinkedList {
            val: Some(val),
            next: Some(Box::new(self)),
        }
    }

    pub fn stringify(&self) -> String {
        match self.val {
            Some(t) => 
                format!("{}, {}", t, t.stringify()),
            None => {format!("Nil")}
        }
    }
}

fn main() {}
