use std::cmp;

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if let Some(median) = Solution::easy_median(&nums1, &nums2) {
            return median;
        }
        
        if let Some(median) = Solution::search_median(&nums1, &nums2) {
            return median;
        }

        if let Some(median) = Solution::search_median(&nums2, &nums1) {
            return median;
        }

        0.0
    }

    pub fn search_median(target: &Vec<i32>, other: &Vec<i32>) -> Option<f64> {
        let (mut i, mut j) = (0 as i32, target.len() as i32);
        let odd_length = (target.len() + other.len()) % 2 == 1;

        while i < j {
            let mid = (i+j)/2;
            let insert_index = Solution::insert_index(other, target[mid as usize]) as i32;

            let nb_smaller = mid + insert_index;
            let nb_greater = target.len() as i32-mid-1 + other.len() as i32-insert_index;

            if odd_length && nb_greater == nb_smaller {
                let mid = mid as usize;
                return Some(target[mid] as f64);
            }
            if !odd_length && (nb_smaller-nb_greater).abs() == 1 {
                let mid = mid as usize;
                let insert_index = insert_index as usize;
                let mut median = target[mid];
                let mut neighbor;

                if nb_smaller > nb_greater {
                    neighbor = i32::MIN;
                    if insert_index > 0 {
                        neighbor = other[insert_index-1];
                    }
                    if mid != 0 {
                        neighbor = cmp::max(neighbor, target[mid-1]);
                    }
                }
                else {
                    neighbor = i32::MAX;
                    if insert_index < other.len() {
                        neighbor = other[insert_index];
                    }
                    if mid != target.len()-1 {
                        neighbor = cmp::min(neighbor, target[mid+1]);
                    }
                }
                
                println!("target={:?}", target);
                println!("mid_index={mid}, insert_index={insert_index}, neighbor={neighbor}");

                median += neighbor;
                return Some((median as f64)/2.0);
            }

            if nb_smaller < nb_greater {
                i = mid+1;
            }
            else {
                j = mid;
            }
        }

        None
    }

    /*
    Return the left-most index where to insert target in nums.
    If target is bigger than the last element of nums, returned
    index will be the size of nums
     */
    pub fn insert_index(nums: &Vec<i32>, target: i32) -> usize {
        let (mut i, mut j) = (0, nums.len());

        while j-i > 1 {
            let mid = (i+j)/2;
            if target == nums[mid] && target != nums[mid-1] {
                return mid;
            }
            else if target <= nums[mid] {
                j = mid;
            }
            else {
                i = mid;
            }
        }
        if target == nums[i] {
            return i;
        }
        if target > nums[i] {
            return i+1;
        }
        0
    }

    /*
    Check trivial cases:
    - One of the array is empty
    - The two arrays are disjoint (expect for edges)
     */
    pub fn easy_median(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Option<f64> {
        if nums1.len() == 0 {
            return Some(Solution::median(nums2));
        }
        if nums2.len() == 0 {
            return Some(Solution::median(nums1));
        }

        let mut all_nums;
        if nums1[nums1.len()-1] <= nums2[0] {
            all_nums = nums1.clone();
            all_nums.append(&mut nums2.clone());
        }
        else if nums2[nums2.len()-1] <= nums1[0] {
            all_nums = nums2.clone();
            all_nums.append(&mut nums1.clone()); 
        }
        else {
            return None;
        }

        Some(Solution::median(&all_nums))
    }

    /*
    Return the median of a sorted array
     */
    pub fn median(nums: &Vec<i32>) -> f64 {
        let mid = nums.len()/2;
        if nums.len() % 2 == 0 {
            return ((nums[mid-1]+nums[mid]) as f64)/2.0;
        }
        nums[mid] as f64
    }
}

fn main() {
    let test_cases = vec![
        vec![vec![], vec![1]],
        vec![vec![1,2], vec![]],
        vec![vec![1,2], vec![3,4]],
        vec![vec![1,3], vec![2]],
        vec![vec![1, 100], vec![1,2]],
        vec![vec![1,1], vec![1,1]],
        vec![vec![1,3], vec![2,7]],
        vec![vec![1,2,2], vec![1,2,3]],
        vec![vec![2,2,4,4], vec![2,2,4,4]],
        vec![vec![2], vec![1,3,4]],
        vec![vec![3], vec![1,3,4]],
        vec![vec![2], vec![1,3,4,5]],
        vec![vec![2], vec![1,3,4,5,6,7]],
        vec![vec![1,3], vec![2,4,5,6,7,8]],
        vec![vec![1,2,4], vec![3]],
        vec![vec![7,8,9], vec![1,2,3,4,5,6,10]]];

    for test in &test_cases {
        println!("{:?} -> {}", 
        test, Solution::find_median_sorted_arrays(test[0].clone(), test[1].clone()));
    }
}
