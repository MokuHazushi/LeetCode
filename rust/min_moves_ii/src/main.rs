// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort();

        let mid_val = nums[nums.len()/2];
        nums.iter().fold(0, |acc, el| acc + (el-mid_val).abs())
    }
}

fn main() {
    println!("Find minimum number of moves to equal array elements");
    let mut test_set = Vec::new();

    test_set.push(vec![1,2,3]);
    test_set.push(vec![1,10,2,9]);

    for test in test_set {
        println!("nums={:?} -> {}", test, Solution::min_moves2(test.clone()));
    }
}
