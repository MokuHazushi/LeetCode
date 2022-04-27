// https://leetcode.com/problems/roman-to-integer/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let symbols = Solution::get_roman_symbols();
        let pair_symbols = Solution::get_pair_symbols();
        let mut chars = s.chars();
        let mut last_char = None;
        let mut ans = 0;


        while let Some(c) = chars.next() {
            if let Some(last_c) = last_char {
                let mut pair = String::from("");
                pair.push(last_c);
                pair.push(c);
                if let Some(val) = pair_symbols.get(&pair) {
                    ans -= symbols.get(&last_c).unwrap();
                    ans += val;
                    last_char = None;
                    continue;
                }
                
            }
            ans += symbols.get(&c).unwrap();
            last_char = Some(c);
        }
        ans
    }

    fn get_roman_symbols() -> HashMap<char, i32> {
        let mut symbols = HashMap::new();

        symbols.insert('I', 1);
        symbols.insert('V', 5);
        symbols.insert('X', 10);
        symbols.insert('L', 50);
        symbols.insert('C', 100);
        symbols.insert('D', 500);
        symbols.insert('M', 1000);

        symbols
    }

    fn get_pair_symbols() -> HashMap<String, i32> {
        let mut pair_symbols = HashMap::new();

        pair_symbols.insert("IV".to_string(), 4);
        pair_symbols.insert("IX".to_string(), 9);
        pair_symbols.insert("XL".to_string(), 40);
        pair_symbols.insert("XC".to_string(), 90);
        pair_symbols.insert("CD".to_string(), 400);
        pair_symbols.insert("CM".to_string(), 900);

        pair_symbols
    }
}

fn main() {
    println!("Convert roman number to int");
    let test_set = vec!["III", "LVIII", "MCMXCIV"];

    for test in test_set {
        println!("{} -> {}", test, Solution::roman_to_int(test.to_string()));
    }

}
