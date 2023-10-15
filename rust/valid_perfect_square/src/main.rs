struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }

        let num: i64 = num.into();
        let mut i: i64 = 1;
        let mut j: i64 = num.into();

        while i < j-1 {
            let mid = i + ((j-i)/2);
            let mid_square = mid * mid;
            if mid_square == num {
                return true;
            }
            else if mid_square < num {
                i = mid+1;
            }
            else {
                j = mid-1;
            }
        }
        i * i == num || j * j == num
    }
}

fn main() {
    let test_set = vec![17, 1, 3, 4, 16, 15, 101];

    for test in &test_set {
        println!("{test} -> {}", Solution::is_perfect_square(*test));
    }
}
