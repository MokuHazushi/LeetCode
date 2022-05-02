// https://leetcode.com/problems/consecutive-numbers-sum/

struct Solution {}

use std::time::Instant;
use std::cmp::Ordering;

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let max = (n as f32/2.0).ceil() as i32;
        let mut ans = {
            if n < 3 {
                0
            }
            else {
                1
            }
        };
        let mut bound = (1, 1);

        while bound.1 <= max+1 {
            let consecutive_sum = Solution::sum(bound.1 as i64) - Solution::sum((bound.0-1) as i64);
            let diff = consecutive_sum - n as i64;

            if diff == 0 {
                ans += 1;
                println!("Found bound {:?}", bound);
                let range = bound.1 - bound.0;
        
                if range <= 1 {
                    break;
                }

                let new_range = n/range;
                if Solution::sum(new_range as i64) - Solution::sum(bound.0 as i64) > (n/2) as i64 {
                    bound.0 += 1;
                }
                else {
                    println!("Nice skip!");
                    bound.0 = new_range;
                }
                bound.1 = bound.0;
            }
            else if diff < 0 { // smaller than n
                bound.1 += 1;
            }
            else { // bigger than n
                bound.0 += 1;
            }
        }

        ans
    }

    // Adapted binary search that returns element just after target
    fn binary_search(bounds: (i32, i32), target: i64) -> i32 {
        let (mut left, mut right);

        // Search upper bound
        left = 0;
        right = bounds.1;
        while left+1 < right {
            let mid = (right-left)/2 + left;
            let mid_val = Solution::sum(mid as i64) - Solution::sum(bounds.0 as i64);

            match mid_val.cmp(&target) {
                Ordering::Less => left = mid,
                Ordering::Greater => right = mid,
                Ordering::Equal => { return mid; },
            }
        }
        right
    }

    fn sum(n: i64) -> i64 {
        ((n*(n+1))/2) as i64
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

    //test_set.push(Test::new(43156417, 4));
    
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(1, Solution::binary_search((0,6), 1), "element = range[0]");
        assert_eq!(3, Solution::binary_search((0,6), 6), "range[0] < element < range[size]");
        assert_eq!(6, Solution::binary_search((0,6), 21), "element = range[size]"); 
        assert_eq!(5, Solution::binary_search((0,6), 11), "range[0] < element < range[size], element not in range"); 
        assert_eq!(6, Solution::binary_search((0,6), 25), "element > range[size]"); 
        assert_eq!(1, Solution::binary_search((1,6), 0), "element < range[0]");
    }
}
