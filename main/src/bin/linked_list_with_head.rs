#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct List<T> {
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
        }
    }

    pub fn len(&self) -> u32 {
        let mut size = 0u32;

        let mut cur_node = &self.head;
        while let Some(node) = cur_node {
            cur_node = &node.next;
            size += 1;
        }
        size
    }
    
    pub fn push_last(&mut self, value: T) {
        let mut cur_node_opt_ref = &mut self.head;
        
        while let Some(cur_node_ref) = cur_node_opt_ref {
            cur_node_opt_ref = &mut cur_node_ref.next;
        }

        *cur_node_opt_ref = Some(Box::new(Node{
            elem: value,
            next: Option::None
        }));
    }

    /// Pops from head
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node: Box<Node<T>>| {
            self.head = node.next;
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
        list.push_last(1); list.push_last(2); list.push_last(3);
        assert_eq!(list.len(), 3);

        list.push_last(1); list.push_last(2); list.push_last(3);
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
    }
}

fn main() {}
