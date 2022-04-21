// https://leetcode.com/problems/first-bad-version/

struct Solution {
    bad_version: i32,
}

impl Solution {

    fn new(bad_version: i32) -> Self {
        Solution {
            bad_version: bad_version,
        }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
		let mut left = 1;
        let mut right = n;

        while left < right {
            let mid = (right-left)/2 + left;

            if !self.isBadVersion(mid) {
                if self.isBadVersion(mid+1) {
                    return mid+1;
                }

                left = mid+1;
            }
            else {
                right = mid;
            }
        }

        left
    }

    fn isBadVersion(&self, n: i32) -> bool {
        n >= self.bad_version
    }
}

fn main() {
    println!("Search first bad version.");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn in_between_bad_version() {
        let solution = Solution::new(10);
        assert_eq!(solution.first_bad_version(20), 10);
    }

    #[test]
    fn first_version_is_bad_version() {
        let solution = Solution::new(1);
        assert_eq!(solution.first_bad_version(20), 1);
    }

    #[test]
    fn last_version_is_bad_version() {
        let solution = Solution::new(20);
        assert_eq!(solution.first_bad_version(20), 20);
    }

    #[test]
    fn one_version() {
        let solution = Solution::new(1);
        assert_eq!(solution.first_bad_version(1), 1);
    }

    #[test]
    fn all_versions() {
        let solution = Solution::new(2147483647);
        assert_eq!(solution.first_bad_version(2147483647), 2147483647);
    }
}
