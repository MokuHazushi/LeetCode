// https://leetcode.com/problems/maximum-erasure-value/

struct Solution {}

use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0,0);
        let mut current = 0;
        let mut max_score = 0;
        let mut used_nums = HashMap::new();

        while right < nums.len() {
            if !used_nums.contains_key(&nums[right]) {
                used_nums.insert(nums[right], true);
                current += nums[right];
                right += 1;
                max_score = cmp::max(max_score, current);
            }
            else {
                used_nums.remove(&nums[left]);
                current -= nums[left];
                left += 1;
            }
        }
        max_score
    }
}

fn main() {
    println!("Find subarray of max value w/ unique elements");
    let mut test_set = Vec::new();

    test_set.push(vec![4,2,4,5,6]);
    test_set.push(vec![5,2,1,2,5,2,1,2,5]);

    for test in test_set {
        println!("nums={:?} -> {}", test, Solution::maximum_unique_subarray(test.clone()));
    }
}
