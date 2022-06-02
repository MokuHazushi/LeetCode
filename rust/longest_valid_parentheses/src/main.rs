// https://leetcode.com/problems/longest-valid-parentheses/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let (mut ans, mut max_ans) = (0,0);
        let mut opened_bracket_left  = 0;

        for c in s.bytes() {
            if c == b'(' {
                opened_bracket_left += 1;
            }
            else {
                if opened_bracket_left == 0 {
                    max_ans = cmp::max(ans, max_ans);
                    ans = 0;
                }
                else {
                    ans += 1;
                    opened_bracket_left -= 1;
                }
            }
        }

        max_ans = cmp::max(ans, max_ans);
        2*max_ans
    }
}

fn main() {
    println!("Length of the longest valid parentheses substring");
    let mut test_set = Vec::new();

    test_set.push(String::from("(()"));
    test_set.push(String::from(")()())"));
    test_set.push(String::from("(()))()"));

    for test in test_set {
        println!("'{}' -> {}", test, Solution::longest_valid_parentheses(test.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_tests() {
        assert_eq!(Solution::longest_valid_parentheses(String::from("(()")), 2);
        assert_eq!(Solution::longest_valid_parentheses(String::from(")()())")), 4);
        assert_eq!(Solution::longest_valid_parentheses(String::from("(()))()")), 4);
        assert_eq!(Solution::longest_valid_parentheses(String::from("())(()")), 2);
        assert_eq!(Solution::longest_valid_parentheses(String::from("((()())()")), 8);
        assert_eq!(Solution::longest_valid_parentheses(String::from(")())(())")), 4);
        assert_eq!(Solution::longest_valid_parentheses(String::from("()(()")), 2);
        assert_eq!(Solution::longest_valid_parentheses(String::from("())(())(()")), 4); // substring is in the middle
        assert_eq!(Solution::longest_valid_parentheses(String::from("()(())(()()")), 6); // substring is at the beginning
        assert_eq!(Solution::longest_valid_parentheses(String::from("()())())(())()")), 6); // substring is at the end
        // other cases to consider: too much ) or too much (
    }
}
