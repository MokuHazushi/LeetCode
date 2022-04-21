// https://leetcode.com/problems/4sum/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut two_sum: HashMap<i32, i32> = HashMap::new();
        let mut three_sum: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        let mut four_sum: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        let mut cleaned_nums: Vec<i32>;

        cleaned_nums = Solution::get_clean_nums(&nums);
        cleaned_nums.sort();

        for i in 0..cleaned_nums.len() {
            let n = cleaned_nums[i];
            let complement = target-n;
            
            match four_sum.get(&complement) {
                Some(all_values) => {
                    for values in all_values {
                        let mut v = vec![n];
                        for val in values {
                            v.push(*val);
                        }

                        if !Solution::values_already_exist(&v, &ans) {
                            ans.push(v);
                        }
                    }
                },
                None => (),
            }

            for (val, all_values) in &three_sum {
                for values in all_values {
                    Solution::insert_sums(&mut four_sum, n+val, values, n);
                }
            }

            for (val, _) in &two_sum {
                let v = vec![*val];
                Solution::insert_sums(&mut three_sum, n+val, &v, n);
            }

            two_sum.insert(n, n);
        }

        ans
    }

    fn insert_sums(map : &mut HashMap<i32, Vec<Vec<i32>>>, key: i32, existing_vals: &Vec<i32>, new_val: i32) {
        let mut v = vec![new_val];
        for i in existing_vals {
            v.push(*i);
        }

        match map.get_mut(&key) {
            Some(values) => {
                if !Solution::values_already_exist(&v, values) {
                    values.push(v)
                }
            },
            None => { map.insert(key, vec![v]); }
        }
    }

    // Assumes vals is sorted and all vectors in all_values are also sorted
    fn values_already_exist(vals: &Vec<i32>, all_values: &Vec<Vec<i32>>) -> bool {
        for values in all_values {
            let mut exists = true;
            for i in 0..values.len() {
                if values[i] != vals[i] {
                    exists = false;
                }
            }

            if exists {
                return true;
            }
        }
        return false;
    }

    fn get_clean_nums(nums: &Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut cleaned_nums = Vec::new();

        for n in nums {
            if map.contains_key(n) {
                if let Some(k) = map.get_mut(n) {

                    if *k < 4 {
                        cleaned_nums.push(*n);
                        *k += 1;
                    }
                }
            }
            else {
                cleaned_nums.push(*n);
                map.insert(*n, 1);
            }
        }

        cleaned_nums
    }
}

struct Test {
    target: i32,
    nums: Vec<i32>,
}

impl Test {
    fn new(target: i32, nums: Vec<i32>) -> Self {
        Test {
            target: target,
            nums: nums,
        }
    }
}

fn main() {
    println!("Find unique quadruplets that sums to target");

    let mut testset: Vec<Test> = Vec::new();

    //testset.push(Test::new(0, vec![1,0,-1,0,-2,2]));
    //testset.push(Test::new(8, vec![2,2,2,2,2]));
    //testset.push(Test::new(0, vec![-2,-1,-1,1,1,2,2]));
    testset.push(Test::new(4, vec![-5,5,4,-3,0,0,4,-2])); // expect only 2 answers

    for test in testset {
        println!("Target={}, nums={:?}", test.target, test.nums);
        println!("\t-> {:?}", Solution::four_sum(test.nums, test.target));
    }
}
