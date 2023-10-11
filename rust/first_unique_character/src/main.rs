struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let s = s.as_bytes();
        let mut letters = [0; 26];

        for i in 0..s.len() {
            letters[(s[i]-97) as usize] += 1;
        }
        
        for i in 0..s.len() {
            if letters[(s[i]-97) as usize] == 1 {
                return i as i32;
            }
        }
        return -1;
    }
}

fn main() {
    let test_set = vec![
        String::from("leetcode"), 
        String::from("loveleetcode"), 
        String::from("aabb")];

    for s in &test_set {
        println!("{s} -> {}", Solution::first_uniq_char(s.to_string()));
    }
}
