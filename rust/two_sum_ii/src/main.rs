// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len()-1;
        
        while i < j {
            match (numbers[i]+numbers[j]).cmp(&target) {
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
                Ordering::Equal => return vec![(i+1) as i32, (j+1) as i32],
            }
        }
        unreachable!();
    }
}

struct Test {
    numbers: Vec<i32>,
    target: i32,
}

impl Test {
    fn new(numbers: Vec<i32>, target: i32) -> Self {
        Test {
            numbers: numbers,
            target: target,
        }
    }
}

fn main() {
    println!("Find i,j so that nums[i]+nums[j] = target");
    let mut test_set: Vec<Test> = Vec::new();

    test_set.push(Test::new(vec![2,7,11,15], 9));
    test_set.push(Test::new(vec![2,3,4], 6));
    test_set.push(Test::new(vec![-1,0], -1));

    for test in test_set {
        println!("Target={} Numbers={:?}", test.target, test.numbers);
        println!("\t-> {:?}", Solution::two_sum(test.numbers, test.target));
    }
}
