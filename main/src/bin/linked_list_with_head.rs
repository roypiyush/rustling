#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct List<T> {
    size: u32,
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        List {
            head: None,
            size: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.size
    }

    pub fn push_back(&mut self, value: T) {
        let mut cur_node_opt_ref = &mut self.head;

        // this is linear operation, but we need constant O(1) operation
        while let Some(cur_node_ref) = cur_node_opt_ref {
            cur_node_opt_ref = &mut cur_node_ref.next;
        }

        *cur_node_opt_ref = Some(Box::new(Node {
            elem: value,
            next: Option::None,
        }));
        self.size += 1;
    }

    /// Pops from head
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node: Box<Node<T>>| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use crate::List;

    #[test]
    fn test_push_pop() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 3);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 6);

        assert_eq!(list.pop_front().unwrap(), 1);
        assert_eq!(list.pop_front().unwrap(), 2);
        assert_eq!(list.pop_front().unwrap(), 3);
        assert_eq!(list.len(), 3);

        assert_eq!(list.pop_front().unwrap(), 1);
        assert_eq!(list.pop_front().unwrap(), 2);
        assert_eq!(list.pop_front().unwrap(), 3);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_scale() {
        let mut list = List::new();

        let size = 100u32;
        for i in 1..=size {
            list.push_back(i); // this is linear insertion
        }

        for i in 1..=size {
            assert_eq!(list.pop_front().unwrap(), i);
        }
    }
}

fn main() {}
