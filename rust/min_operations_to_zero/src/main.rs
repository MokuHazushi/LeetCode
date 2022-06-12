// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let nums_sum = {
            let mut s = 0;
            for n in &nums {
                s += n;
            }
            s
        };
        let (mut left, mut right) = (0, 0);
        let mut current = 0;
        let mut ans = -1;

        if nums_sum == x {
            return nums.len() as i32;
        }

        while right < nums.len() {
            current += nums[right];

            while current > nums_sum - x && left <= right {
                current -= nums[left];
                left += 1;
            }

            if current == nums_sum - x {
                ans = cmp::max(ans, (right - left) as i32 + 1);
            }
            right += 1;
        }
        if ans == -1 {
            -1
        }
        else {
            nums.len() as i32 - ans
        }
    }
}

struct Test {
    nums: Vec<i32>,
    x: i32,
}

impl Test {
    fn new(nums: Vec<i32>, x: i32) -> Self {
        Test {
            nums: nums,
            x: x,
        }
    }
}

fn main() {
    println!("Find minimum number of operations to reduce x to zero");
    let mut test_set = Vec::new();

    // Solution exists
    test_set.push(Test::new(vec![1,1,4,2,3], 5));
    test_set.push(Test::new(vec![1,1,3,4,4,3], 5));
    test_set.push(Test::new(vec![3,2,20,1,1,3], 10));
    test_set.push(Test::new(vec![6,6,20,1,3], 4));
    test_set.push(Test::new(vec![1,1,1,1], 4));

    // Solution does not exists
    test_set.push(Test::new(vec![5,6,7,8,9], 4));
    test_set.push(Test::new(vec![5,6,7,8,9,10], 4));
    test_set.push(Test::new(vec![1,1,4,2,3], 100));
    test_set.push(Test::new(vec![1,1,4,2,3,3], 100));

    // Special tabs
    test_set.push(Test::new(vec![4], 4));
    test_set.push(Test::new(vec![4], 3));
    test_set.push(Test::new(vec![2,2], 4));
    test_set.push(Test::new(vec![2,3], 4));

    // Leetcode failed tests
    test_set.push(Test::new(vec![4,1,1,1], 4));
    test_set.push(Test::new(vec![4,1,1,1,1,1], 4));

    for test in test_set {
        println!("nums={:?}, x={}\n\t-> {}", test.nums, test.x, Solution::min_operations(test.nums.clone(), test.x));
    }
}
