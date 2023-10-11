use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut indexes = HashMap::new();
        let mut dim = 2;
        let mut start = 1;
        let mut ans = vec![vec![]];
        for i in 0..nums.len() {
            indexes.insert(nums[i], i);
            ans.push(vec![nums[i]]);
        }
        while dim <= nums.len() {
            let mut new_sets = Solution::subsets_dim(&nums, &ans[start..], &indexes);
            start = ans.len();
            ans.append(&mut new_sets);
            dim += 1;
        }
        ans
    }

    /**
     * Given a subsets with the same number of elements (same dimension), return the
     * subsets of elements of dimension+1.
     * Example:
     * nums=[1,2,3]
     * susbets=[(1,2), (1,3), (2,3)]
     * Returns = [(1,2,3)]
     */
    pub fn subsets_dim(nums: &Vec<i32>, subsets: &[Vec<i32>], indexes: &HashMap<i32, usize>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        for subset in subsets {
            let highest_index = indexes.get(&subset[subset.len()-1]).unwrap();

            for i in (highest_index+1)..nums.len() {
                let mut set = subset.clone();
                set.push(nums[i]);
                ans.push(set);
            }
        }
        ans
    }
}

fn main() {
    let test_sets = vec![
        vec![1,2,3], 
        vec![0], 
        vec![1,2,3,4]];

    for test in &test_sets {
        println!("{:?} -> {:?}", test, Solution::subsets(test.clone()));
    }
}
