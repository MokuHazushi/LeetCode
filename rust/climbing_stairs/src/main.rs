// https://leetcode.com/problems/climbing-stairs/

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memory = HashMap::new();

        memory.insert(1, 1);
        memory.insert(2, 2);

        Solution::dp(n, &mut memory)
    }

    fn dp(n: i32, memory: &mut HashMap<i32, i32>) -> i32 {
        if let Some(ans) = memory.get(&n) {
            *ans
        }
        else {
            let recursion1 = Solution::dp(n-1, memory);
            let recursion2 = Solution::dp(n-2, memory);
            memory.insert(n, recursion1+recursion2);
            *memory.get(&n).unwrap()
        }
    }
}

fn main() {
    println!("Climbing stairs!");

    for i in 1..=10 {
        println!("{} stairs -> {}", i, Solution::climb_stairs(i));
    }
}
