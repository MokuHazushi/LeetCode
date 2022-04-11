use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut i = 0;
        let mut j = x;
        let max = 46340;

        if x <= 1 {
            return x
        }

        while j-i > 1 {
            let mid = i + (j-i)/2;

            if mid > max {
                j = mid;
                continue;
            }

            match (mid*mid).cmp(&x) {
                Ordering::Less => i = mid,
                Ordering::Greater => j = mid,
                Ordering::Equal => return mid
            }
        }
        i
    }
}

fn main() {
    println!("Computing truncated square-root");
    let test_set = vec![2147395599];

    for n in test_set {
        println!("sqrt({})={}", n, Solution::my_sqrt(n));
    }
}
