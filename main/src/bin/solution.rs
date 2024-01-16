struct Solution;

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

            if Self::get_value(&nums, &m, &size) == target  {
                break;
                
            } else if Self::get_value(&nums, &m, &size) < target 
                    && target < Self::get_value(&nums, &(m + 1), &size) {
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
}

fn main() {
    let mut nums = vec![1,2];

    assert_eq!(nums.remove(0), 1);
    assert_eq!(nums.remove(0), 2);

    let target = 2;
    println!("{}", Solution::search_insert(nums, target));
}