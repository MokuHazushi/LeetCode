// https://leetcode.com/problems/powx-n/

struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n >= 0 {
            Solution::my_positive_pow(x, n as i64)
        }
        else {
            1./Solution::my_positive_pow(x, -(n as i64))
        }
    }

    fn my_positive_pow(x: f64, n: i64) -> f64 {
        let mut memory = VecDeque::from([x]);
        let mut n = n;
        let mut pow = 1;
        let mut ans = 1.;

        while n > 0 {
            if n >= pow {
                let back = *memory.back().unwrap();
                n -= pow;
                ans *= back;
                memory.push_back(back*back);
                pow = pow << 1;
                
            }
            else {
                memory.pop_back();
                pow = pow >> 1;
            }
        }
        ans
    }
}

fn main() {
    println!("Implement pow!");
    let test_set = vec![
        (2.0, -2147483648),
        (2.0, 10), (3.0,4), (3.0, 5), (3.0, 6),
        (2.5, 5), (-2.5, 5),
        (5., 0),
        (3.0, -4), (3.0, -5), (3.0, -6),
        ];

    for test in test_set {
        println!("pow({}, {}) -> {}", test.0, test.1, Solution::my_pow(test.0, test.1));
    }
}
