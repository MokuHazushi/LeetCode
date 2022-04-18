// https://leetcode.com/problems/ransom-note/

use std::collections::HashMap;

struct Solution {}

#[derive(Debug)]
struct Test {
    ransom_note: String,
    magazine: String,
}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters: HashMap<char, i32> = HashMap::new();
        
        for letter in magazine.chars() {
            match letters.get_mut(&letter) {
                Some(val) => { *val = *val+1; },
                None => { letters.insert(letter, 1); },
            };
        }

        for letter in ransom_note.chars() {
            match letters.get_mut(&letter) {
                Some(val) => {
                    if *val <= 0 {
                        return false;
                    }
                    *val = *val-1;
                },
                None => return false,
            };
        }

        return true
    }
}

impl Test {
    pub fn new(ransom_note: String, magazine: String) -> Test {
        Test {
            ransom_note: ransom_note, 
            magazine: magazine
        }
    }
}

fn main() {
    let test_set = vec![
        Test::new(String::from("aa"), String::from("aab")),
        Test::new(String::from("aa"), String::from("ab"))
        ];

    println!("Checking if 'ransom_note' can be built from letters in 'magazine':");
    for test in test_set {
        println!("Input = {:?}", &test);
        println!("\t-> {}", Solution::can_construct(test.ransom_note, test.magazine));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_corresponding_letters_in_magazine() {
        let test = Test::new(String::from("a"), String::from("b"));
        assert_eq!(Solution::can_construct(test.ransom_note, test.magazine), false);
    }

    #[test]
    fn exactly_corresponding_letters_in_magazine() {
        let test = Test::new(String::from("abc"), String::from("abc"));
        assert_eq!(Solution::can_construct(test.ransom_note, test.magazine), true);
    }

    #[test]
    fn not_enough_letters_in_magazine() {
        let test = Test::new(String::from("aaa"), String::from("b"));
        assert_eq!(Solution::can_construct(test.ransom_note, test.magazine), false);
    }
}
