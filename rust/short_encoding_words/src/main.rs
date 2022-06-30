// https://leetcode.com/problems/short-encoding-of-words/

struct Solution {}

struct DicEntry {
    letters: Vec<Option<Box<DicEntry>>>
}

impl DicEntry {
    pub fn new() -> Self {
        let mut letters = Vec::new();
        for _ in 0..26 {
            letters.push(None);
        }
        DicEntry {
            letters: letters,
        }
    }
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut dic = DicEntry::new();
        let mut words = words;
        let mut ans = 0;

        words.sort_by(|w1, w2| w1.len().cmp(&w2.len()));
        words.reverse();
        
        for word in words {
            let mut dic_iter = &mut dic;
            let letters = word.as_bytes();
            let mut i = letters.len()-1;
            while i > 0 {
                let index = (letters[i]-97) as usize;
                if dic_iter.letters[index].is_none() {
                    dic_iter.letters[index] = Some(Box::new(DicEntry::new()));
                }
                dic_iter = dic_iter.letters[index].as_mut().unwrap().as_mut();
                i -= 1;
            }

            let index = (letters[i]-97) as usize;
            if dic_iter.letters[index].is_none() {
                ans += letters.len()+1;
                dic_iter.letters[index] = Some(Box::new(DicEntry::new()));
            }
        }
        ans as i32
    }
}

fn main() {
    println!("Find min length for encoding words");
    let mut test_set = Vec::new();

    test_set.push(vec![String::from("time"), String::from("me"), String::from("bell")]);

    for test in test_set {
        println!("words={:?} -> {}", test.clone(), Solution::minimum_length_encoding(test));
    }
}
