struct Solution {}

// https://leetcode.com/problems/search-in-rotated-sorted-array/

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
        let mut min = 0;
        let mut max = nums.len()-1;

        while min <= max {
            let middle = (max-min)/2 + min;

            if nums[middle] == target {
                return middle as i32
            }
            
            if nums[min] <= nums[middle] {
                // nums[min..middle+1] is ordered

                if target >= nums[min] && target < nums[middle] {
                    max = middle-1;
                    continue;
                }
                else {
                    min = middle+1;
                    continue;
                }
            }

            if nums[middle] <= nums[max] {
                // nums[middle..max+1] is ordered
                if target > nums[middle] && target <= nums[max] {
                    min = middle+1;
                    continue;
                }
                else {
                    max = middle-1;
                    continue
                }
            }
        }
        -1
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
