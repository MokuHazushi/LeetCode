struct Solution {}

impl Solution {
    pub fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let tmp = nums[i];
        nums[i] = nums[j];
        nums[j] = tmp;
    }

    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut swap_pos = 0;

        while i < nums.len() {
            if nums[i] != 0 {
                if i != swap_pos {
                    Solution::swap(nums, i, swap_pos);
                }
                swap_pos += 1;
            }
            i += 1;
        }
    }
}


fn main() {
    let mut test_set = vec![
        vec![0, 1, 0, 3, 12],
        vec![0, 0, 0, 1],
        vec![1, 2, 3, 4],
        vec![0, 0, 0]
        ];

    for test in &mut test_set {
        let original = test.clone();
        Solution::move_zeroes(test);
        println!("{:?} -> {:?}", original, test);
    }
}
