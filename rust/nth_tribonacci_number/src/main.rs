// https://leetcode.com/problems/n-th-tribonacci-number/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut memory = vec![0; cmp::max(3, n+1)];

        memory[0] = 0;
        memory[1] = 1;
        memory[2] = 1;

        for i in 3..=n {
            memory[i] = memory[i-1] + memory[i-2] + memory[i-3];
        }

        memory[n]
    }
}

fn main() {
    println!("Compute tribonacci number!");
    let test_set = vec![0,1,2,3,4,10,20,30];

    for test in test_set {
        println!("n={} -> {}", test, Solution::tribonacci(test));
    }
}
