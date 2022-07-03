// https://leetcode.com/problems/wiggle-subsequence/

struct Solution {}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let (mut current_sign, mut current_sign_set) = (false, false);
        let mut ans = 1;

        for i in 1..nums.len() {
            if nums[i-1] == nums[i] {
                continue;
            }

            if !current_sign_set {
                current_sign = nums[i-1] < nums[i];
                ans += 1;
                current_sign_set = true;
            }
            else if current_sign != (nums[i-1] < nums[i]) {
                ans += 1;
                current_sign = nums[i-1] < nums[i];
            }
        }

        ans
    }
}

fn main() {
    println!("Find length of longest wiggle subsequence");
    let mut test_set = Vec::new();

    test_set.push(vec![1,7,4,9,2,5]);
    test_set.push(vec![1,17,5,10,13,15,10,5,16,8]);
    test_set.push(vec![1,2,3,4,5,6,7,8,9]);

    for test in test_set {
        println!("nums={:?} -> {}", test.clone(), Solution::wiggle_max_length(test));
    }
}
