// https://leetcode.com/problems/number-of-matching-subsequences/

struct Solution {}

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut heads: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 26];
        let mut ans = 0;
        
        for i in 0..words.len() {
            heads[(words[i].as_bytes()[0]-97) as usize].push((i, 0));
        }
        
        for c in s.as_bytes() {
            heads.push(Vec::new());
            let pairs = heads.swap_remove((c-97) as usize);

            for pair in pairs {
                let word = words[pair.0].as_bytes();
                if word.len()-1 == pair.1 {
                    ans += 1;
                    continue;
                }
                heads[(word[pair.1+1]-97) as usize].push((pair.0, pair.1+1));
            }
        }      
        ans
    }
}

fn main() {
    println!("Count number of matching subsequences");
    let mut test_set = Vec::new();

    test_set.push(("abcde".to_string(), 
        vec!["a".to_string(),
            "bb".to_string(),
            "acd".to_string(),
            "ace".to_string()]));

    for test in test_set {
        println!("s={}, words={:?} -> {}", test.0, test.1, Solution::num_matching_subseq(test.0.clone(), test.1.clone()));
    }
}
