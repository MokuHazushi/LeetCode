use std::cmp;

struct Solution {}

impl Solution {
    /*
    We want to use 'nums' as a hashmap where:
    - 'nums[i] == 0' means integer 'i+1' was not in original input array
    - 'nums[i] == -1' means integer 'i+1' was in original input array (regardless of number of occurences)
    */
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        // Clean-up input
        let mut nums: Vec<_> = nums.iter().map(|&x| cmp::max(x, 0)).collect();
        
        for i in 0..nums.len() {
            let key = nums[i];

            // We already processed this key
            if key <= 0 {
                continue;
            }

            nums[i] = 0; // Indicate the key is processed
            Solution::put(&mut nums, key);
        }

        // Search for the solution in the newly populated hashmap
        for i in 0..nums.len() {
            if nums[i] == 0 {
                return i as i32 + 1;
            }
        }
        nums.len() as i32 + 1
    }

    fn put(map: &mut Vec<i32>, key: i32) {
        /*
        - 'key <= 0' means 'key' is actually an already assigned value in the map
        - 'key > map.len()' means 'key' is bigger than the solution space
        */
        if key <= 0 || key > map.len() as i32 {
            return;
        }

        let hash = (key-1) as usize;

        // map[hash] can be overwritten
        if map[hash] <= 0 {
            map[hash] = -1;
            return;
        }

        // map[hash] value should be handled through a recursive call
        let next_key = map[hash];
        map[hash] = -1;
        Solution::put(map, next_key);
    }
}

fn main() {
    let test_cases = vec![
        vec![1,2,0],
        vec![3,4,-1,1],
        vec![7,8,9,11,12],
        vec![7,8,9],
        vec![2],
        vec![2,2],
        vec![1, 1000],
        vec![2, 1]
    ];

    for test in &test_cases {
        println!("{:?} -> {}", test, Solution::first_missing_positive(test.clone()));
    }
}
