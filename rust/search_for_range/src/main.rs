// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {

        if nums.len() == 0 {
            return vec![-1,-1];
        }

        if nums.len() == 1 {
            if nums[0] == target {
                return vec![0,0];
            }
            else {
                return vec![-1,-1];
            }
        }

        vec![Solution::search_lower_bound(&nums, target), Solution::search_upper_bound(&nums, target)]      
    }

    fn search_lower_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len()-1);

        while left < right {
            let mid = (right-left)/2 + left;

            if target <= nums[mid] {
                right = mid;
            }
            else {
                left = mid+1;
            }
        }
        if nums[left] == target {
            left as i32
        }
        else {
            -1
        }
    }

    fn search_upper_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len()-1);

        while left+1 < right {
            let mid = (right-left)/2 + left;

            if target >= nums[mid] {
                left = mid;
            }
            else {
                right = mid;
            }
        }
        if nums[right] == target { // Checking right first is necessary for case when nums last element is target
            right as i32
        }
        else if nums[left] == target {
            left as i32
        }
        else {
            -1
        }
    }
}

struct Test {
    nums: Vec<i32>,
    target: i32,
}

impl Test {
    fn new(nums: Vec<i32>, target: i32) -> Self {
        Test {
            nums: nums,
            target: target,
        }
    }
}

fn main() {
    println!("Search for a range:");
    let mut test_set = Vec::new();

    test_set.push(Test::new(vec![5,7,7,8,8,10], 8));
    test_set.push(Test::new(vec![], 8));
    test_set.push(Test::new(vec![1], 8));
    test_set.push(Test::new(vec![8], 8));
    test_set.push(Test::new(vec![1,8], 8));
    test_set.push(Test::new(vec![1,8], 1));
    test_set.push(Test::new(vec![8,8], 8));
    test_set.push(Test::new(vec![1,8,8,8,8,8,8,8,8,8,8,9], 8));
    test_set.push(Test::new(vec![1,2,3], 0));
    test_set.push(Test::new(vec![2,2],3));
    test_set.push(Test::new(vec![2,2],1));
    test_set.push(Test::new(vec![5,7,7,8,8,10], 5));
    test_set.push(Test::new(vec![5,7,7,8,8,10], 10));
    test_set.push(Test::new(vec![5,7,7,8,8,10], 6));

    for test in test_set {
        println!("target={}, tab={:?}", test.target, test.nums);
        println!("\t-> {:?}", Solution::search_range(test.nums, test.target));
    }
}
