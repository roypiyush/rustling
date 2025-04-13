#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1_opt = list1;
        let mut list2_opt = list2;

        let mut result = None;
        let mut ptr = &mut result;

        loop {
            match (&list1_opt, &list2_opt) {
                (None, None) => {
                    break;
                }
                (Some(_), None) => {
                    let l = list1_opt.take();
                    *ptr = Some(l.unwrap());
                }
                (None, Some(_)) => {
                    let l = list2_opt.take();
                    *ptr = Some(l.unwrap());
                }
                (Some(l1), Some(l2)) => {
                    if l1.val < l2.val {
                        let l = list1_opt.take();
                        let mut list_node = l.unwrap();
                        list1_opt = list_node.next.take();
                        *ptr = Some(list_node);
                    } else {
                        let l = list2_opt.take();
                        let mut list_node = l.unwrap();
                        list2_opt = list_node.next.take();
                        *ptr = Some(list_node);
                    }
                }
            }
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        result
    }
}

fn main() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    println!("{:?}", Solution::merge_two_lists(list1, list2));
}
