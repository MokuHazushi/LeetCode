// https://leetcode.com/problems/longest-substring-without-repeating-characters/

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut start, mut end, mut max_length) = (0, 0, 0);
        let mut used_characters = HashMap::new();
        let s = s.as_bytes();

        while end < s.len() {
            if start == end || !used_characters.contains_key(&s[end]) {
                used_characters.insert(s[end], true);
                end += 1;
            }
            else {
                used_characters.remove(&s[start]);
                start += 1;
            }
            if end-start > max_length {
                max_length = end-start;
            }
        }
        max_length as i32
    }
}

fn main() {
    println!("Find size of the longest substring w/o repeating characters");
    let mut test_set = Vec::new();

    test_set.push("abcabcbb".to_string());
    test_set.push("bbbbbb".to_string());
    test_set.push("pwwkew".to_string());

    for test in test_set {
        println!("s={} -> {}", test, Solution::length_of_longest_substring(test.clone()));
    }
}
