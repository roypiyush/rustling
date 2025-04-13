use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: u32,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T>
where
    T: std::fmt::Debug,
{
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn print(&self) {
        // getting a new pointer to same allocation
        let mut current_ptr = self.head.clone();
        while let Some(cur_node) = current_ptr {
            let borrowed_node = cur_node.borrow();
            print!("{:?} -> ", borrowed_node.elem);
            current_ptr = borrowed_node.next.clone();
        }
        println!("NIL");
    }

    pub fn len_by_while(&self) -> u32 {
        let mut len = 0u32;
        let mut current = self.head.clone();

        while let Some(node) = current {
            len += 1;
            current = node.borrow().next.clone();
        }

        len
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |n| &n.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |n| &n.elem))
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(old_node) => {
                old_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_node);
                self.tail = Some(new_node);
            }
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.head.take() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(old_node) => {
                old_node.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_node);
                self.head = Some(new_node);
            }
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                None => {
                    self.tail.take();
                    self.len -= 1;
                }
                Some(next_node) => {
                    next_node.borrow_mut().prev.take();
                    self.head = Some(next_node);
                    self.len -= 1;
                }
            }
            Rc::try_unwrap(old_head).unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail: Rc<RefCell<Node<T>>>| {
            match old_tail.borrow_mut().prev.take() {
                None => {
                    self.head.take();
                    self.len -= 1;
                }
                Some(next_node) => {
                    next_node.borrow_mut().next.take();
                    self.tail = Some(next_node);
                    self.len -= 1;
                }
            }
            Rc::try_unwrap(old_tail).unwrap().into_inner().elem
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
        assert_eq!(&*list.peek_back().unwrap(), &1);

        list.push_back(2);
        assert_eq!(&*list.peek_back().unwrap(), &2);

        list.push_back(3);
        assert_eq!(&*list.peek_back().unwrap(), &3);

        assert_eq!(list.len(), 3);

        assert_eq!(list.len(), list.len_by_while());

        list.print();

        assert_eq!(*list.peek_front().unwrap(), 1);
        assert_eq!(*list.peek_back().unwrap(), 3);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len(), 6);

        list.print();

        // assert_eq!(list.len_by_node_count(), 6);

        assert_eq!(list.pop_front().unwrap(), 1);
        assert_eq!(list.pop_front().unwrap(), 2);
        assert_eq!(list.pop_front().unwrap(), 3);
        assert_eq!(list.len(), 3);

        list.print();

        assert_eq!(list.pop_front().unwrap(), 1);
        assert_eq!(list.pop_front().unwrap(), 2);
        assert_eq!(list.pop_front().unwrap(), 3);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_front(), None);

        list.print();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);

        assert_eq!(list.pop_back().unwrap(), 1);
        assert_eq!(list.pop_back().unwrap(), 2);
        assert_eq!(list.pop_back().unwrap(), 3);
        assert_eq!(list.len(), 0);
        list.pop_back();
        assert_eq!(list.len(), 0);
        list.pop_back();
        assert_eq!(list.len(), 0);
        list.pop_back();
        assert_eq!(list.len(), 0);
    }
}

fn main() {}
