use std::cmp::Ordering;


struct Solution {}

impl Solution {
    pub fn search(nums: &Vec<i32>, target: i32) -> i32 {
        let mut range = (0, nums.len()-1);

        while range.1 - range.0 > 1 {
            let middle = ((range.1 - range.0)/2) + range.0;

            match nums[middle].cmp(&target) {
                Ordering::Less => range.0 = middle+1,
                Ordering::Greater => range.1 = middle-1,
                Ordering::Equal => return middle as i32,
            }
        }

        if nums[range.0] == target {
            range.0 as i32
        }
        else if nums[range.1] == target {
            range.1 as i32
        }
        else {
            -1
        }
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
