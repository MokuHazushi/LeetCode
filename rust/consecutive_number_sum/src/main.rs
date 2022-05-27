// https://leetcode.com/problems/consecutive-numbers-sum/

struct Solution {}

use std::time::Instant;

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut r = 0;
        let max_div = (((2*n) as f32).sqrt().ceil() + 1.0) as i32;

        for n1 in 1..max_div {
            if n % n1 == 0 {
                let tmp = n - (n1*(n1+1)/2);
                if tmp % n1 == 0 {
                    r += 1;
                }
            }
        }
        r+1
    }
}

struct Test {
    n: i32,
    expect: i32,
}

impl Test {
    fn new(n: i32, expect: i32) -> Self {
        Test {
            n: n,
            expect: expect,
        }
    }
}

fn main() {
    println!("How many ways to write n with consecutive numbers?");
    let mut test_set = Vec::new();

    test_set.push(Test::new(1, 1));
    test_set.push(Test::new(2, 1));
    test_set.push(Test::new(3, 2));
    test_set.push(Test::new(4,1));
    test_set.push(Test::new(5, 2));
    test_set.push(Test::new(9, 3));
    test_set.push(Test::new(7,2));
    test_set.push(Test::new(15, 4));
    test_set.push(Test::new(23, 2));
    test_set.push(Test::new(45, 6));
    test_set.push(Test::new(43156417, 4));
    
    for test in test_set {
        let ans = Solution::consecutive_numbers_sum(test.n);
        if ans == test.expect {
            println!("OK n={} -> {}", test.n, ans);
        }
        else {
            println!("ERROR n={} -> {}, expected {}", test.n, ans, test.expect);
        }
    }
    
    
    println!("Computation time test, compute for n=65581200");
    let now = Instant::now();
    let res = Solution::consecutive_numbers_sum(65581200);
    println!("Found {}, Took {:?}", res, now.elapsed());
    
    
}
