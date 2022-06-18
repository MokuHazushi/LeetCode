// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut memory = vec![vec![0; multipliers.len()+1]; multipliers.len()+1];
        
        for i in 1..=multipliers.len() {
            for j in 0..=i {
                if j == 0 { // right elements only
                    memory[i][j] = memory[i-1][j] + (multipliers[i-1] * nums[nums.len()-i]);
                    continue;
                }
                if j == i { // left elements only
                    memory[i][j] = memory[i-1][j-1] + (multipliers[i-1] * nums[i-1]);
                    continue;
                }
                
                memory[i][j] = cmp::max(
                    memory[i-1][j-1] + (multipliers[i-1] * nums[j-1]),
                    memory[i-1][j] + (multipliers[i-1] * nums[nums.len()-i+j]))
            }
        }

        *memory[multipliers.len()].iter().max().unwrap()
    }
}

struct Test {
    nums: Vec<i32>,
    multipliers: Vec<i32>,
}

impl Test {
    pub fn new(nums: Vec<i32>, multipliers: Vec<i32>) -> Self {
        Test {
            nums: nums,
            multipliers: multipliers,
        }
    }
}

fn main() {
    println!("Maximum score using multiplications");
    let mut test_set = Vec::new();

    test_set.push(Test::new(vec![1,2,3], vec![3,2,1]));
    test_set.push(Test::new(vec![-5,-3,-3,-2,7,1], vec![-10,-5,3,4,6]));

    for test in test_set {
        println!("nums={:?}, multipliers={:?}", &test.nums, &test.multipliers);
        println!("\t-> {}", Solution::maximum_score(test.nums, test.multipliers));
    }
}
