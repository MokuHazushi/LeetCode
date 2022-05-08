// https://leetcode.com/problems/3sum/

struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        let mut triplets = Vec::new();
        let mut i;

        if nums.len() < 3 {
            return triplets;
        }

        nums.sort();
        i = 0;
        while i < nums.len()-2 && nums[i] < 1 {
            if i > 0 && nums[i-1] == nums[i] {
                i += 1;
                continue;
            }
            Solution::two_pointers_search(&nums[i+1..], nums[i], &mut triplets);
            i += 1;
        }
        
        triplets
    }

    fn two_pointers_search(nums: &[i32], target: i32, triplets: &mut Vec<Vec<i32>>) {
        let mut left = 0;
        let mut right = nums.len()-1;

        while left < right {
            match (nums[left]+nums[right]+target).cmp(&0) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => {
                    triplets.push(vec![target, nums[left], nums[right]]);
                    while left < nums.len()-1 && nums[left+1] == nums[left] {
                        left += 1;
                    }
                    left += 1;
                    right -= 1;
                },
            }
        }
    }
}

fn main() {
    println!("Find triplets who sum to 0 (no duplicates)");
    let mut test_set = Vec::new();

    test_set.push(vec![-1,0,0,0,1,2,3,-1,-4]);
    test_set.push(vec![0,0,0]);
    test_set.push(vec![1,1,1]);
    test_set.push(vec![-1,-1,-1]);
    test_set.push(vec![1]);
    test_set.push(vec![]);
    test_set.push(vec![1,1,-2]);
    test_set.push(vec![-4,-3,-2]);
    test_set.push(vec![6,-5,-6,-1,-2,8,-1,4,-10,-8,-10,-2,-4,-1,-8,-2,8,9,-5,-2,-8,-9,-3,-5]);
    test_set.push(vec![-1,-4,-5,2,-2,-1,-6,-10,-10,-9,2,3,2,2,-1,4,-9,-1,-4,6,3,-2,-8,-4,5,-3,-9]);

    for test in test_set {
        println!("nums={:?}\n\t-> {:?}", test.clone(), Solution::three_sum(test));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(Solution::two_pointers_search(&vec![0,1,1,2,2,3], 3), vec![vec![0,3], vec![1,2]]);
    }
}