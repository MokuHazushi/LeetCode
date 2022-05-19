// https://leetcode.com/problems/longest-common-subsequence/

struct Solution {}

use std::cmp;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        /*
        Solution::dp_topdown(
            0, 0, 
            text1.as_bytes(), text2.as_bytes(), 
            &mut vec![vec![0; text2.len()]; text1.len()])        
        */
        Solution::dp_bottomup(text1.as_bytes(), text2.as_bytes())
    }

    fn dp_topdown(
        i: usize, j: usize, 
        text1: &[u8], text2: &[u8],
        memory: &mut Vec<Vec<i32>>) -> i32 {

        if i == text1.len() || j == text2.len() {
            return 0;
        }

        if memory[i][j] == 0 {
            if text1[i] == text2[j] {
                memory[i][j] = 1 + Solution::dp_topdown(i+1, j+1, text1, text2, memory);
            }
            else {
                memory[i][j] = cmp::max(
                    Solution::dp_topdown(i+1, j, text1, text2, memory), 
                    Solution::dp_topdown(i, j+1, text1, text2, memory));
            }
        }
        memory[i][j]
    }

    fn dp_bottomup(text1: &[u8], text2: &[u8]) -> i32 {
        let mut memory = vec![vec![0; text2.len()+1]; text1.len()+1];

        for i in 0..text1.len() {
            for j in 0..text2.len() {
                if text1[i] == text2[j] {
                    memory[i+1][j+1] = 1 + memory[i][j];
                }
                else {
                    memory[i+1][j+1] = cmp::max(memory[i][j+1], memory[i+1][j]);
                }
            }
        }

        memory[text1.len()][text2.len()]
    }
}

fn main() {
    println!("Find longest common subsequence");
    let mut test_set = Vec::new();

    test_set.push((String::from("abcde"), String::from("ace")));
    test_set.push((String::from("abc"), String::from("abc")));
    test_set.push((String::from("abc"), String::from("def")));
    test_set.push((String::from("abcdeza"), String::from("acezza")));

    for test in test_set {
        println!("text1={} text2={} -> {}", 
            &test.0, &test.1, 
            Solution::longest_common_subsequence(test.0.clone(), test.1.clone()));
    }
}
