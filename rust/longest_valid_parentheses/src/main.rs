// https://leetcode.com/problems/longest-valid-parentheses/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let (mut left_stack, mut right_stack) = (0, 0);
        let mut longest_seq = 0;
        
        for c in s.bytes() {
            if c == b'(' {
                left_stack += 1;
            }
            else {
                right_stack += 1;
            }
            
            if left_stack == right_stack {
                longest_seq = cmp::max(longest_seq, left_stack);
            }
            
            if right_stack > left_stack {
                left_stack = 0;
                right_stack = 0;
            }
        }
        
        left_stack = 0;
        right_stack = 0;
        
        for c in s.bytes().rev() {
            if c == b'(' {
                left_stack += 1;
            }
            else {
                right_stack += 1;
            }
            
            if left_stack == right_stack {
                longest_seq = cmp::max(longest_seq, left_stack);
            }
            
            if left_stack > right_stack {
                left_stack = 0;
                right_stack = 0;
            }
            
        }
        
        2*longest_seq        
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
