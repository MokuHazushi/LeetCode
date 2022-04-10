use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;

struct Solution {}

struct RowValue {
    soldiers: i32,
    index: usize,
}

impl Ord for RowValue {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.soldiers != other.soldiers {
            self.soldiers.cmp(&other.soldiers)
        }
        else {
            self.index.cmp(&other.index)
        }
    }
}

impl PartialOrd for RowValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RowValue {
    fn eq(&self, other: &Self) -> bool {
        self.soldiers == other.soldiers &&
        self.index == other.index
    }
}

impl Eq for RowValue {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {

        let mut heap = BinaryHeap::new();
        let mut result: Vec<i32> = Vec::new();
        let mut i: usize = 0;

        while i < mat.len() {
            let mut soldiers = 0;
            for el in &mat[i] {
                if *el == 1 {
                    soldiers += 1;
                }
            }
            let tmp: RowValue = RowValue {
                soldiers: soldiers,
                index: i,
            };
            heap.push(Reverse(tmp));
            i += 1;
        }

        for _i in 0..k {
            match heap.pop() {
                Some(index) => {
                    result.push(index.0.index.try_into().unwrap())
                }
                None => {},
            }
        }
        
        result
    }
}

fn main() {
    println!("Looking for k-weakest rows");
    let mut test: Vec<Vec<i32>> = Vec::new();

    test.push(vec![1,1,0,0,0]);
    test.push(vec![1,1,1,1,0]);
    test.push(vec![1,0,0,0,0]);
    test.push(vec![1,1,0,0,0]);
    test.push(vec![1,1,1,1,1]);

    let result = Solution::k_weakest_rows(test, 3);
    println!("Result is {:?}", result);
}
