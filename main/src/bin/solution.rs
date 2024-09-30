use std::{cell::RefCell, rc::Rc};

struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn get_value(nums: &Vec<i32>, index: &i32, size: &i32) -> i32 {
        if index < &0 {
            i32::MIN
        } else if index >= size {
            i32::MAX
        } else {
            nums[*index as usize]
        }
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let size = nums.len() as i32;

        let mut s = 0;
        let mut e = size as i32 - 1;

        let mut m = s + (e - s) / 2;
        while s <= e {
            m = s + (e - s) / 2;

            if Self::get_value(&nums, &m, &size) == target {
                break;
            } else if Self::get_value(&nums, &m, &size) < target
                && target < Self::get_value(&nums, &(m + 1), &size)
            {
                break;
            } else if target < Self::get_value(&nums, &m, &size) {
                e = m - 1;
            } else {
                s = m + 1;
            }
        }

        if Self::get_value(&nums, &m, &size) == target {
            m
        } else {
            m + 1
        }
    }

    fn find_insert_pos(nums: &Vec<i32>, insert_pos: usize, elem: i32) -> usize {
        let mut counter = insert_pos;
        let size = nums.len();

        while counter < size {
            if nums[counter] > elem {
                break;
            }
            counter += 1;
        }
        counter
    }

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>) {
        nums1.resize(m as usize, 0);

        let mut insert_pos: usize = 0;
        while nums2.len() > 0 {
            let elem = nums2.remove(0);
            insert_pos = Self::find_insert_pos(nums1, insert_pos, elem);
            nums1.insert(insert_pos, elem);
        }
    }

    pub fn length_of_last_word(s: String) -> i32 {
        let mut len = 0;

        for char_at in s.chars().rev() {
            if len > 0 && char_at == ' ' {
                return len;
            } else if char_at == ' ' {
            } else {
                len += 1;
            }
        }
        len
    }

    pub fn bool_compare(match_vec: &Vec<char>, needle: &String) -> bool {
        let mut chars = needle.chars();

        for c in match_vec {
            let result = match chars.next() {
                None => false,
                Some(v) => c == &v,
            };
            if !result {
                return result;
            }
        }
        true
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle_size = needle.len();
        let haystack_size = haystack.len();
        let mut i = 0;
        let mut j = 0;

        let mut match_vec: Vec<char> = Vec::new();
        let h_chars: Vec<char> = haystack.chars().collect();

        while j < haystack_size {
            match_vec.push(h_chars[j]);
            if j >= needle_size {
                match_vec.remove(0);
                i += 1;
            }

            if match_vec.len() == needle_size && Self::bool_compare(&match_vec, &needle) {
                return i;
            }

            j += 1;
        }

        -1
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node_ref) => {
                let mut borrow_mut = node_ref.borrow_mut();

                let left = borrow_mut.left.take();
                let right = borrow_mut.right.take();

                borrow_mut.left = Self::invert_tree(right);
                borrow_mut.right = Self::invert_tree(left);

                return Some(node_ref.clone());
            }
        }
    }

    /// This is returning a new reversed element. "head" list is dropped when function ends.
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // convert to mutable as full ownership is present
        let mut head = head;

        // pointer, wrapper element to be mutable
        let mut ref_head = head.as_mut();

        // result list
        let mut result = None;

        // remove first element
        while let Some(node_ref) = ref_head.take() {
            // get mutable reference to next element
            let next_node = node_ref.next.as_mut();

            // create node with next = result (prev) and assign prev to as current (result)
            result = Some(Box::new(ListNode {
                val: node_ref.val,
                next: result,
            }));

            // set reference to next node, which will be first element in remaining list
            ref_head = next_node;
        }

        // at this moment "result" and "head" exists.
        // can we reduce this memory overhead

        result
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut size = 0;
        let mut ptr = &head;
        while let Some(n) = ptr {
            size += 1;
            ptr = &n.next;
        }

        if size == 1 || size == 0 {
            return true;
        }

        let (list1, list2) = Self::break_into_half(size, head);

        let mut list1 = list1.as_ref();
        let mut list2 = list2.as_ref();

        while let (Some(l1), Some(l2)) = (list1, list2) {
            if l1.val != l2.val {
                return false;
            }

            list1 = l1.next.as_ref();
            list2 = l2.next.as_ref();
        }

        true
    }

    fn break_into_half(
        size: i32,
        head: Option<Box<ListNode>>,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let half = size / 2;

        let mut list2 = head;
        let mut list1 = None;

        let mut count = 0;
        while let Some(node_ref) = list2.take() {
            let next_node = node_ref.next;
            list1 = Some(Box::new(ListNode {
                val: node_ref.val,
                next: list1,
            }));

            list2 = next_node;

            count += 1;

            if count >= half {
                if size % 2 == 1 {
                    if let Some(node_ref) = list2.take() {
                        list2 = node_ref.next;
                    }
                }
                break;
            }
        }

        // list1 is also reversed
        (list1, list2)
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut path: Vec<i32> = Vec::new();
        let mut result: Vec<String> = Vec::new();
        Self::internal_binary_tree_paths(&root, &mut path, &mut result);
        result
    }

    fn internal_binary_tree_paths(
        root: &Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<i32>,
        result: &mut Vec<String>,
    ) {
        if let Some(node_ref) = root {
            let val = node_ref.borrow().val;
            let left = &node_ref.borrow().left;
            let right = &node_ref.borrow().right;

            path.push(val);

            if left.is_none() && right.is_none() {
                result.push(
                    path.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("->"),
                );
            } else {
                if left.is_some() {
                    Self::internal_binary_tree_paths(&left, path, result);
                }
                if right.is_some() {
                    Self::internal_binary_tree_paths(&right, path, result);
                }
            }

            path.remove(path.len() - 1);
        }
    }
}

fn main() {
    let s = String::from("Hello World");
    let mut charst = s.chars();
    charst.nth(0);
    let mut nums = vec![1, 2];

    ListNode::new(0);
    assert_eq!(nums.remove(0), 1);
    assert_eq!(nums.remove(0), 2);

    let target = 2;
    println!("{}", Solution::search_insert(nums, target));

    let ref mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let ref mut nums2 = vec![2, 5, 6];

    Solution::merge(nums1, 3, nums2);
    println!("{:?}", nums1);

    println!(
        "{:?}",
        Solution::length_of_last_word(String::from("hello world"))
    );
    println!(
        "{:?}",
        Solution::str_str(String::from("leetcode"), String::from("leeto"))
    );

    Solution::reverse_list(None);
    Solution::invert_tree(None);
    Solution::is_palindrome(None);
    Solution::binary_tree_paths(None);
}
