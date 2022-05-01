// https://leetcode.com/problems/consecutive-numbers-sum/

struct Solution {}

use std::time::Instant;

use std::collections::HashMap;

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let max = (n as f32/2.0).ceil() as i32;
        let mut ans = 1;
        let mut added_numbers: HashMap<i64, bool> = HashMap::new();

        if n == 1 {
            return 1;
        }

        for i in 0..=max {
            let i = i as i64;
            let sum = (i*(i+1))/2;

            if added_numbers.contains_key(&(sum+n as i64)) || added_numbers.contains_key(&(sum-n as i64)) {
                ans += 1;
            }
            added_numbers.insert(sum, true);
        }

        ans
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
    test_set.push(Test::new(15, 4));
    

    for test in test_set {
        let ans = Solution::consecutive_numbers_sum(test.n);
        if ans == test.expect {
            println!("OK n={} -> {}", test.n, ans);
        }
        else {
            println!("ERROR n={} -> {}, expected {}", test.n, ans, test.expect);
        }
    }

    println!("Trying to find some pattern...");

    for i in 1..=36 {
        println!("n={} -> {}", i, Solution::consecutive_numbers_sum(i));
    }

    println!("Computation time test, compute for n=8504986");
    let now = Instant::now();
    Solution::consecutive_numbers_sum(8504986);
    println!("Took {:?}", now.elapsed());
}
