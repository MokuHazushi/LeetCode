use std::cmp::Ordering;


struct Solution {}

impl Solution {
    pub fn search(nums: &Vec<i32>, target: i32) -> i32 {
        let mut range: (usize, usize) = (0, nums.len()-1);
        
        while range.1-range.0 > 1 {
            let middle = ((range.1-range.0)/2)+range.0;
            match nums[middle].cmp(&target) {
                Ordering::Less => {
                    range.0 = middle+1;
                    continue;
                }
                Ordering::Greater => {
                    range.1 = middle-1;
                    continue;
                }
                Ordering::Equal => return middle.try_into().unwrap(),
            }
        }

        if nums[range.0] == target {
            return range.0.try_into().unwrap()
        }
        else if nums[range.1] == target {
            return range.1.try_into().unwrap()
        }

        -1
    }
}

fn main() {
    println!("Binary search in a sorted list");
    let test: Vec<i32> = vec![1,2,3,4,5];

    for i in 0..7 {
        print!("Search '{}' in {:?}", i, &test);
        println!("\t found at i={}", Solution::search(&test, i));
    }
}
