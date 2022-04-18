// https://leetcode.com/problems/guess-number-higher-or-lower/

struct Solution {}
impl Solution {
    fn guessNumber(n: i32) -> i32 {
        let mut max = n;
        let mut min = 1;
        let mut middle: i32;
        loop {
            middle = (max-min)/2 + min;
            match guess(middle) {
                0 => return middle,
                -1 => max = middle-1,
                1 =>
                    min = middle+1,
                _ => {}
            }
        }
    }
}

fn guess(num: i32) -> i32 {
    let n = 50;

    if num == n {
        0
    }
    else if num < n {
        1
    }
    else {
        -1
    }
}

fn main() {
    println!("Guessing number using binary search!");
    let test_set: Vec<i32> = vec![(2_i64.pow(31)-1) as i32];

    for n in test_set {
        println!("Guessed {}", Solution::guessNumber(n));
    }
}
