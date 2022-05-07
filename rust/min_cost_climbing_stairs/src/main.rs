// https://leetcode.com/problems/min-cost-climbing-stairs/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut memory = vec![-1; cost.len()];

        if cost.len() == 2 {
            return cmp::min(cost[0], cost[1]);
        }

        memory[cost.len()-1] = cost[cost.len()-1];
        memory[cost.len()-2] = cost[cost.len()-2];
        for i in (0..=cost.len()-3).rev() {
            memory[i] = cost[i] + cmp::min(memory[i+1], memory[i+2]);
        }

        cmp::min(memory[0], memory[1])
    }
}

fn main() {
    println!("Minimal cost to climb stairs?");

    let mut test_set = Vec::new();

    test_set.push(vec![1]);
    test_set.push(vec![2,1]);
    test_set.push(vec![10,15,20]);
    test_set.push(vec![1,100,1,1,1,100,1,1,100,1]);
    test_set.push(vec![1,100,1,1,1,100,1,1,100]);
    test_set.push(vec![1,1,1,1,1,1]);

    for test in test_set {
        println!("cost={:?} -> {}", test.clone(), Solution::min_cost_climbing_stairs(test));
    }
}