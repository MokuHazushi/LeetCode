// https://leetcode.com/problems/house-robber/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut memory = vec![-1;nums.len()];
        memory[0] = nums[0];
        memory[1] = cmp::max(nums[0], nums[1]);

        for i in 2..nums.len() {
            memory[i] = cmp::max(memory[i-1], memory[i-2]+nums[i]);
        }
        
        memory[nums.len()-1]
    }
}

fn main() {
    println!("Find the best benefit you can get!");
    let mut test_set = Vec::new();

    test_set.push(vec![1,2,3,1]);
    test_set.push(vec![1,2]);
    test_set.push(vec![1]);

    for test in test_set {
        println!("houses={:?} -> {}", test.clone(), Solution::rob(test));
    }
}
