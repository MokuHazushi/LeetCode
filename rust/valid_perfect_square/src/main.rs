struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        true
    }
}

fn main() {
    let test_set = vec![1, 4, 16, 15];

    for test in &test_set {
        println!("{test} -> {}", Solution::is_perfect_square(*test));
    }
}
