// https://leetcode.com/problems/fizz-buzz/

struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans = Vec::with_capacity(n as usize);

        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                ans.push(String::from("FizzBuzz"));
                continue;
            }
            if i % 3 == 0 {
                ans.push(String::from("Fizz"));
                continue;
            }
            if i % 5 == 0 {
                ans.push(String::from("Buzz"));
                continue;
            }
            ans.push(i.to_string());
        }
        ans        
    }
}

fn main() {
    println!("FizzBuzz!");

    for i in 1..16 {
        println!("n={} -> {:?}", i, Solution::fizz_buzz(i));
    }
}
