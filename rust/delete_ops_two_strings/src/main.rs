// https://leetcode.com/problems/delete-operation-for-two-strings/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();

        let mut memory = vec![vec![0 as i32; word2.len()+1]; word1.len()+1];

        for i in 0..word1.len() {
            for j in 0..word2.len() {
                if word1[i] == word2[j] {
                    memory[i+1][j+1] = 1 + memory[i][j];
                }
                else {
                    memory[i+1][j+1] = cmp::max(memory[i][j+1], memory[i+1][j]);
                }
            }
        }

        word1.len() as i32 + word2.len() as i32 - (2 * memory[word1.len()][word2.len()])
    }
}

fn main() {
    println!("Distance between 2 words");
    let mut test_set = Vec::new();

    test_set.push(("sea".to_string(), "eat".to_string()));
    test_set.push(("leetcode".to_string(), "etco".to_string()));
    test_set.push(("letcode".to_string(), "etco".to_string()));
    test_set.push(("abc".to_string(), "cba".to_string()));
    test_set.push(("abc".to_string(), "bca".to_string()));
    test_set.push(("aabbcc".to_string(), "bca".to_string()));
    test_set.push(("abc".to_string(), "def".to_string()));
    test_set.push(("abcde".to_string(), "bcdae".to_string()));

    for test in test_set {
        println!("({}, {}) -> {}", test.0, test.1, Solution::min_distance(test.0.clone(), test.1.clone()));
    }
}
