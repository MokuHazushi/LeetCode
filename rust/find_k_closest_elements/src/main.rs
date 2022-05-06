// https://leetcode.com/problems/find-k-closest-elements/

struct Solution {}

use std::cmp::Ordering;
use std::cmp;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut x_pos = Solution::binary_search(&arr, x);
        let mut ans = Vec::new();
        
        if arr.len()-1-x_pos > x_pos {
            // Fill from start of arr
            let k = k as usize;
            for i in cmp::max(0, x_pos-(k/2))..x_pos {
                ans.push(arr[i]);
            }
            while ans.len() < k {
                ans.push(arr[x_pos]);
                x_pos += 1;
            }
        }
        else {
            // Fill from end of arr
            let k = k as usize;
            for i in (x_pos..cmp::min(arr.len(), x_pos+(k/2))).rev() {
                ans.push(arr[i]);
            }
            while ans.len() < k {
                ans.push(arr[x_pos]);
                x_pos -= 1;
            }
            ans.reverse();
        }

        ans
    }

    fn binary_search(arr: &Vec<i32>, x: i32) -> usize {
        let (mut left, mut right) = (0, arr.len()-1);
        while left+1 < right {
            let middle = (right-left)/2 + left;
            match x.cmp(&arr[middle]) {
                Ordering::Less => right = middle,
                Ordering::Greater => left = middle,
                Ordering::Equal => return middle,
            }
        }
        if (x-arr[right]).abs() < (x-arr[left]).abs() {
            right
        }
        else {
            left
        }
    }
}

#[derive(Debug)]
struct Test {
    arr: Vec<i32>,
    k: i32,
    x: i32,
}

impl Test {
    fn new(arr: Vec<i32>, k: i32, x: i32) -> Self {
        Test {
            arr: arr,
            k: k,
            x: x,
        }
    }
}

fn main() {
    println!("Find k integers closest to x in sorted array");
    let mut test_set: Vec<Test> = Vec::new();

    
    test_set.push(Test::new(vec![1,2,3,4,5], 4, 3));
    test_set.push(Test::new(vec![1,2,3,4,5], 4, -1));
    test_set.push(Test::new(vec![1,2,3,4,5], 4, 6));

    test_set.push(Test::new(vec![1,2,3,4,5], 3, 3));
    test_set.push(Test::new(vec![1,2,3,4,5], 3, -1));
    test_set.push(Test::new(vec![1,2,3,4,5], 3, 6));

    test_set.push(Test::new(vec![1,2,3,4,5], 1, 3));
    test_set.push(Test::new(vec![1,2,3,4,5], 1, -1));
    test_set.push(Test::new(vec![1,2,3,4,5], 1, 6));

    test_set.push(Test::new(vec![1,2,4,5], 3, 3));
    test_set.push(Test::new(vec![1,2,3,4,5], 3, 5));
    test_set.push(Test::new(vec![1,2,3,4,5], 3, 1));

    for test in test_set {
        println!("{:?}\n\t->{:?}", test, Solution::find_closest_elements(test.arr.clone(), test.k, test.x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let test = Test::new(vec![1,2,3,4,5], 4, 3);
        assert_eq!(vec![1,2,3,4], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,3,4,5], 4, -1);
        assert_eq!(vec![1,2,3,4], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,3,4,5], 4, 6);
        assert_eq!(vec![2,3,4,5], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,3,4,5], 3, 3);
        assert_eq!(vec![2,3,4], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,3,4,5], 3, -1);
        assert_eq!(vec![1,2,3], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,3,4,5], 3, 6);
        assert_eq!(vec![3,4,5], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,3,4,5], 1, 3);
        assert_eq!(vec![3], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,3,4,5], 1, -1);
        assert_eq!(vec![1], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,3,4,5], 1, 6);
        assert_eq!(vec![5], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,3,4,5], 3, 5);
        assert_eq!(vec![3,4,5], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![1,2,4,5], 3, 3);
        assert_eq!(vec![1,2,4], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![-1,1,2,3,4,5], 7, 3);
        assert_eq!(vec![-2,-1,1,2,3,4,5], Solution::find_closest_elements(test.arr, test.k, test.x));
    }
}
