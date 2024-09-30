#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
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

    pub fn push(&mut self, element: T) {
        let new_node = Node {
            elem: element,
            // replace with put Empty into head and return head which is assigned to next
            next: self.head.take(), //short for std::mem::replace(&mut self.head, Option::None),
        };
        // fix above replacement by assign new value
        // owner of self will still have valid ownership
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node: Box<Node<T>>| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|x| &mut x.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let now = std::time::Instant::now();
        println!("Dropping list len = {}", self.len());

        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }

        println!(
            "Dropped list len = {}, elasped time = {}ms",
            self.len(),
            now.elapsed().as_millis()
        );
    }
}

impl<T> Iterator for List<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

fn main() {
    let mut list = List::new();

    let size = 100; //u32::MAX;
    for i in 0u32..size {
        list.push(i);
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn test_push_pop() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.peek(), Some(&mut 1));

        list.push(2);
        assert_eq!(list.peek(), Some(&mut 2));

        list.push(3);
        assert_eq!(list.peek(), Some(&mut 3));

        assert_eq!(list.len(), 3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);

        assert_eq!(list.peek(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_pop_general() {
        let mut list = List::new();
        let size = 99999u32;
        for i in 0u32..size {
            list.push(i);
        }

        assert_eq!(list.len(), 99999);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
    }
}
