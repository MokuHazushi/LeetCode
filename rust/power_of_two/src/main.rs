struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n.count_ones() == 1
    }
}

fn main() {
    let test_set = vec![-1, 0, 1, 2, 3, 16, -16];

    for test in &test_set {
        println!("{test} -> {}", Solution::is_power_of_two(*test));
    }
}
