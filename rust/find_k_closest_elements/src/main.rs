// https://leetcode.com/problems/find-k-closest-elements/

struct Solution {}

use std::cmp::Ordering;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let x_pos = Solution::binary_search(&arr, x);
        let mut ans = Vec::new();
        let (mut bound_left, mut bound_right) = (x_pos as i32, x_pos as i32);
        let mut go_left;
        
        while bound_right - bound_left + 1 < k {
            let next_left = bound_left - 1;
            let next_right = bound_right + 1;

            if next_left < 0 {
                go_left = false;
            }
            else if next_right > (arr.len()-1) as i32 {
                go_left = true;
            }
            else if (arr[next_left as usize] - x).abs() <= (arr[next_right as usize] - x).abs() {
                go_left = true;
            }
            else {
                go_left = false;
            }

            if go_left {
                if bound_left > 0 {
                    bound_left -= 1;
                }
            }
            else {
                if bound_right < (arr.len()-1) as i32 {
                    bound_right += 1;
                }
            }
        }

        for i in bound_left..=bound_right {
            ans.push(arr[i as usize]);
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

        let test = Test::new(vec![-2,-1,1,2,3,4,5], 7, 3);
        assert_eq!(vec![-2,-1,1,2,3,4,5], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![0,1,1,1,2,3,6,7,8,9], 9, 4);
        assert_eq!(vec![0,1,1,1,2,3,6,7,8], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![0,0,1,2,3,3,4,7,7,8], 3, 5);
        assert_eq!(vec![3,3,4], Solution::find_closest_elements(test.arr, test.k, test.x));

        let test = Test::new(vec![0,0,0,1,3,5,6,7,8,8], 2, 2);
        assert_eq!(vec![1,3], Solution::find_closest_elements(test.arr, test.k, test.x));
    }
}
