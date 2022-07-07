// https://leetcode.com/problems/interleaving-string/

struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let mut memory = vec![vec![false; s2.len()+1]; s1.len()+1];
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();

        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        
        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                if i == 0 && j == 0 {
                    memory[i][j] = true;
                    continue;
                }

                if i == 0 {
                    memory[i][j] = memory[i][j-1] && s2[j-1] == s3[j-1];
                    continue;
                }

                if j == 0 {
                    memory[i][j] = memory[i-1][j] && s1[i-1] == s3[i-1];
                    continue;
                }

                memory[i][j] = 
                    (memory[i-1][j] && s1[i-1] == s3[j+i-1]) ||
                    (memory[i][j-1] && s2[j-1] == s3[j+i-1]);
            }
        }
        memory[s1.len()][s2.len()]
    }
}

fn main() {
    println!("Can s3 is formed by interleaving s1 and s2?");
    let mut test_set = Vec::new();

    test_set.push(("aabcc".to_string(), "dbbca".to_string(), "aadbbcbcac".to_string()));

    for test in test_set {
        println!("s1={}, s2={}, s3={}\n\t->{}", 
        test.0.clone(), test.1.clone(), test.2.clone(), 
        Solution::is_interleave(test.0, test.1, test.2));
    }
}
