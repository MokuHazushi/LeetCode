struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut ans1 = Solution::search(&nums, 1);
        let mut ans2 = Solution::search(&nums, ans1);

        while ans1 != ans2 {
            ans1 = ans2;
            ans2 = Solution::search(&nums, ans1);
        }
        ans1
    }

    pub fn search(nums: &Vec<i32>, start: i32) -> i32 {
        let (mut missing, mut i) = (start, 0);

        while i < nums.len() {
            if nums[i] == missing || nums[nums.len()-1-i] == missing {
                missing += 1;
                continue;
            }
            i += 1;
        }
        missing
    }
}

fn main() {
    let test_cases = vec![
        vec![1,2,0],
        vec![3,4,-1,1],
        vec![7,8,9,11,12],
        vec![7,8,9]
    ];

    for test in &test_cases {
        println!("{:?} -> {}", test, Solution::first_missing_positive(test.clone()));
    }
}
