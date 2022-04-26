// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len()-1);

        while right-left > 1 {
            let mid = (right-left)/2 + left;

            if nums[left] < nums[mid] {
                if nums[mid] < nums[right] {
                    return nums[left];
                }
                left = mid+1;
            }
            else {
                right = mid;
            }
        }

        if nums[left] < nums[right] {
            nums[left]
        }
        else {
            nums[right]
        }
    }
}

fn main() {
    println!("Find minimal element in sorted and rotated array");
    let mut test_set = Vec::new();

    test_set.push(vec![1,2,3,4,5]);
    test_set.push(vec![2,3,4,5,1]);
    test_set.push(vec![3,4,5,1,2]);
    test_set.push(vec![4,5,1,2,3]);
    test_set.push(vec![5,1,2,3,4]);
    test_set.push(vec![2,3,4,5,6,1]);
    

    for test in test_set {
        println!("array={:?} -> {}", test.clone(), Solution::find_min(test));
    }
}
