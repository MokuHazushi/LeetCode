// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let mut memory = vec![vec![0; nums.len()]; multipliers.len()];
        Solution::maximum_score_dp(0, 0, &nums, &multipliers, &mut memory)   
    }

    fn maximum_score_dp(
        i: usize, 
        left: usize, 
        nums: &Vec<i32>, 
        multipliers: &Vec<i32>, 
        memory: &mut Vec<Vec<i32>>) -> i32 {
        if i == multipliers.len() {
            return 0;
        }

        let right = nums.len() - 1 - (i - left);

        if memory[i][left] == 0 {
            memory[i][left] = cmp::max(
                multipliers[i]*nums[left] + Solution::maximum_score_dp(i+1, left+1, nums, multipliers, memory),
                multipliers[i]*nums[right] + Solution::maximum_score_dp(i+1, left, nums, multipliers, memory));
        }
        memory[i][left]
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

    for test in test_set {
        println!("nums={:?}, multipliers={:?}", &test.nums, &test.multipliers);
        println!("\t-> {}", Solution::maximum_score(test.nums, test.multipliers));
    }
}
