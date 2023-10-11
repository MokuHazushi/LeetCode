struct Solution {}

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let s = s.clone() + &s.clone();
        s.contains(&goal)
    }
}

fn main() {
    let test_cases = vec![
        (String::from("abcde"), String::from("cdeab")),
        (String::from("abcde"), String::from("abced")),
        (String::from("abcde"), String::from("ab"))];

    for test in &test_cases {
        println!("{:?} -> {}", test, Solution::rotate_string(test.0.to_string(), test.1.to_string()));
    }
}
