use std::cmp;

struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let (mut min, mut max) = (i32::MAX, i32::MIN);
        let mut positives = 0;
        for num in &nums {
            if *num <= 0 {
                continue;
            }
            positives += 1;
            min = cmp::min(min, *num);
            max = cmp::max(max, *num);
        }

        let holes = max-min+1-positives;

        if holes == 0 {
            if min == 1 {
                return max+1;
            }
            return 1;
        }

        0
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
