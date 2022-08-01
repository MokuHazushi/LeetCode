// https://leetcode.com/problems/unique-paths/

struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut mat = vec![vec![0; n as usize]; m as usize];
        
        for i in 0..m as usize {
            for j in 0..n as usize {
                if i == 0 && j == 0 {
                    mat[i][j] = 1;
                    continue;
                }
                
                if j != 0 {
                    mat[i][j] += mat[i][j-1];
                }
                if i != 0 {
                    mat[i][j] += mat[i-1][j];
                }
            }
        }
        mat[(m-1) as usize][(n-1) as usize]
    }
}

fn main() {
    println!("Number of unique paths");

    let mut test_set = Vec::new();

    test_set.push((3,2));
    test_set.push((3,3));
    test_set.push((3,7));

    for test in test_set {
        println!("{:?} -> {}", test, Solution::unique_paths(test.0, test.1));
    }
}
