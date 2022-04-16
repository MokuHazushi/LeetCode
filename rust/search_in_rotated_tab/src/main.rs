use std::cmp::Ordering;

struct Solution {}

struct Test {
    nums: Vec<i32>,
    target: i32,
}

impl Test {
    pub fn new(nums: Vec<i32>, target: i32) -> Test {
        Test {
            nums: nums,
            target: target,
        }
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let pivot = Solution::find_pivot(&nums[..]);

        if target >= nums[0] {
            if let Some(ans) = Solution::binary_search(&nums[..(pivot+1) as usize], target) {
                ans
            }
            else {
                -1
            }
        }
        else {
            if let Some(ans) = Solution::binary_search(&nums[(pivot+1) as usize..], target) {
                ans+pivot+1
            }
            else {
                -1
            }
        }
    }

    pub fn binary_search(nums: &[i32], target: i32) -> Option<i32> {
        let mut min = 0;
        let mut max = (nums.len()-1) as i32;

        while min <= max {
            let middle = (max-min)/2 + min;
            match target.cmp(&nums[middle as usize]) {
                Ordering::Less => max = middle-1,
                Ordering::Greater => min = middle+1,
                Ordering::Equal => return Some(middle),
            }
        }
        None
    }

    pub fn find_pivot(nums: &[i32]) -> i32 {
        let mut min = 0;
        let mut max = nums.len() as i32;
        while min < max {
            let middle = (max-min)/2 + min;

            if nums[min as usize] < nums[middle as usize] {
                min = middle;
            }
            else {
                max = middle;
            }
        }
        min
    }
}

fn main() {
    println!("Searching target in ordered but shifted array");
    let test_set = vec![
        Test::new(vec![1,2,0], 1),
        Test::new(vec![1,2,0], 2),
        Test::new(vec![1,2,0], 0),
        Test::new(vec![4,5,6,7,0,1,2], 0),
        Test::new(vec![4,5,6,7,0,1,2], 3)
        ];

    for test in test_set {
        println!("Search target={} in array={:?}", test.target, test.nums);
        println!("\t-> {}", Solution::search(test.nums, test.target));
    }
}
