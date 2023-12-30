#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
struct Node {
    elem: u32,
    next: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, element: u32) {
        let new_node = Node {
            elem: element,
            // replace with put Empty into head and return head which is assigned to next
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        // fix above replacement by assign new value
        // owner of self will still have valid ownership
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<u32> {
        let cur_node = std::mem::replace(&mut self.head, Link::Empty);
        match cur_node {
            Link::Empty => Option::None,
            Link::More(node) => {
                self.head = node.next;
                Option::Some((*node).elem)
            }
        }
    }

    pub fn peek(&self) -> Option<u32> {
        match &self.head {
            Link::Empty => Option::None,
            Link::More(node) => Option::Some((*node).elem),
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_push_pop() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.peek(), Some(1));
        
        list.push(2);
        assert_eq!(list.peek(), Some(2));

        list.push(3);
        assert_eq!(list.peek(), Some(3));

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

        assert_eq!(list.peek(), None);
    }

    #[test]
    fn test_push_pop_at_scale() {
        let mut list = List::new();
        let size = 999u32;
        for i in 0u32..size {
            list.push(i);
        }

        for _ in 0u32..size {
            list.pop();
        }

        assert_eq!(list.peek(), None);
    }
}
