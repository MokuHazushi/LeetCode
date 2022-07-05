// https://leetcode.com/problems/longest-consecutive-sequence/

struct Solution {}

use std::cmp;

struct Sequence {
    key: i32,
    sequence: Vec<bool>,
    offset: i32,
}

impl Sequence {
    fn new(key: i32, len: usize) -> Self {
        let mut sequence = vec![false; 2*len - 1];
        let offset = (sequence.len() as i32)/2 - key;
        sequence[(offset + key) as usize] = true;
        Sequence {
            key: key,
            sequence: sequence,
            offset: offset,
        }
    }

    fn set_if_in_range(&mut self, num: i32) -> bool {
        let len = (self.sequence.len()/2) as i32;
        if num >= self.key - len && num <= self.key + len {
            self.sequence[(self.offset + num) as usize] = true;
            true
        }
        else {
            false
        }
    }

    fn longest_sequence(&self) -> i32 {
        let mut longest = 0;
        let mut i = 0;

        while i < self.sequence.len() {
            let mut cur_seq = 0;
            while i < self.sequence.len() && self.sequence[i] {
                cur_seq += 1;
                i += 1;
            }
            longest = cmp::max(cur_seq, longest);
            i += 1;
        }
        longest
    }
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut sequences: Vec<Sequence> = Vec::new();
        let mut ans = 0;

        for num in &nums {
            let mut added = false;
            for sequence in sequences.iter_mut() {
                added |= sequence.set_if_in_range(*num);
            }

            if !added {
                sequences.push(Sequence::new(*num, nums.len()));
            }
        }
        
        for sequence in sequences {
            ans = cmp::max(ans, sequence.longest_sequence());
        }
        ans
    }
}

fn main() {
    println!("Find the longest consecutive sequence");
    let mut test_set = Vec::new();

    test_set.push(vec![100,4,200,1,3,2]);
    test_set.push(vec![0,3,7,2,5,8,4,6,0,1]);
    test_set.push(vec![0,0,0,1,1,1]);
    test_set.push(vec![0,3,2]);
    test_set.push(vec![0]);
    test_set.push(vec![1,0,-1]);

    for test in test_set {
        println!("nums={:?} -> {}", test.clone(), Solution::longest_consecutive(test));
    }
}
