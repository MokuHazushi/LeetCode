use std::collections::HashMap;

// https://leetcode.com/problems/two-sum/

struct Solution {}

impl Solution {
    pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for i in 0..nums.len() {
            if let Some(val) = map.get(&(target - nums[i])) {
                if i != *val {
                    return vec![i as i32, *val as i32]
                }
            }
            map.insert(nums[i], i);
        }

        Vec::new()
    }
}

fn main() {
    println!("Finding (i,j) in vector nums such that nums[i]+nums[j] = target (i != j)");
    let test_set: Vec<(i32, Vec<i32>)> = vec![
        (9, vec![2,7,11,15]),
        (6, vec![3,2,4]),
        (6, vec![3,3])
        ];
    
    for test in &test_set {
        println!("target={} vec={:?} -> {:?}", test.0, test.1, Solution::two_sum(&test.1, test.0));
    }
}
