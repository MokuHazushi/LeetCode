// https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/

struct Solution {}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct SearchEl {
    pos: (usize, usize),
    val: i32,
}

impl SearchEl {
    pub fn new(pos: (usize, usize), val: i32) -> Self {
        SearchEl {
            pos: pos,
            val: val,
        }
    }
}

impl Ord for SearchEl {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for SearchEl {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.val.cmp(&self.val))
    }
}

impl Eq for SearchEl {}

impl PartialEq for SearchEl {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut k = k;
        let mut matrix = matrix;

        heap.push(SearchEl::new((0,0), matrix[0][0]));

        while k > 1 {
            let el = heap.pop().unwrap();
            

            let neighbors = vec![(el.pos.0+1, el.pos.1), (el.pos.0, el.pos.1+1)];
            for neighbor in neighbors {
                if neighbor.0 >= matrix.len() || neighbor.1 >= matrix[0].len() {
                    continue;
                }
                let new_val = matrix[neighbor.0][neighbor.1];

                if new_val >= el.val {
                    heap.push(SearchEl::new(neighbor, new_val));
                    matrix[neighbor.0][neighbor.1] = i32::MIN;
                }
            }
            k -= 1;
        }
        
        heap.peek().unwrap().val
    }
}


fn main() {
    println!("Find kth smallest element in a sorted matrix");

    let mut test_set = Vec::new();

    test_set.push(vec![
        vec![1,5,9],
        vec![10,11,13],
        vec![12,13,15]]);

    for test in test_set {
        let nb_el = test.len()*test[0].len();
        println!("mat={:?}", test.clone());
        for i in 0..nb_el {
            println!("i={} -> {}", i+1, Solution::kth_smallest(test.clone(), (i+1) as i32));
        }
    }
}
