// https://leetcode.com/problems/search-a-2d-matrix-ii/

struct Solution {}

struct Range {
    topleft: (usize, usize),
    botright: (usize, usize),
}

impl Range {
    fn new(topleft: (usize, usize), botright: (usize, usize)) -> Self {
        Range {
            topleft: topleft,
            botright: botright,
        }
    }
}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut ranges = vec![Range::new((0, 0), (matrix.len()-1, matrix[0].len()-1))];

        while !ranges.is_empty() {
            let range = ranges.pop().unwrap();

            if !Solution::in_range(&matrix, target, &range) {
                continue;
            }

            if matrix[range.topleft.0][range.topleft.1] == target {
                return true;
            }

            let mid_row = (range.botright.0 - range.topleft.0)/2 + range.topleft.0;
            let mid_col = (range.botright.1 - range.topleft.1) /2 + range.topleft.1;

            let range1 = Range::new(range.topleft, (mid_row, mid_col));
            let range2 = Range::new((range.topleft.0, mid_col+1), (mid_row, range.botright.1));
            let range3 = Range::new((mid_row+1, range.topleft.1), (range.botright.0, mid_col));
            let range4 = Range::new((mid_row+1, mid_col+1), range.botright);

            if Solution::in_range(&matrix, target, &range1) {
                ranges.push(range1);
            }
            if Solution::in_range(&matrix, target, &range2) {
                ranges.push(range2);
            }
            if Solution::in_range(&matrix, target, &range3) {
                ranges.push(range3);
            }
            if Solution::in_range(&matrix, target, &range4) {
                ranges.push(range4);
            }
        }
        false
    }

    fn in_range(matrix: &Vec<Vec<i32>>, target: i32, range: &Range) -> bool {
        range.topleft.0 < matrix.len() && range.topleft.1 < matrix[0].len() &&
        range.botright.0 < matrix.len() && range.botright.1 < matrix[0].len() &&
        target >= matrix[range.topleft.0][range.topleft.1] && 
        target <= matrix[range.botright.0][range.botright.1]
    }
}


fn main() {
    println!("Search in sorted 2d matrix");
    let test_set1 = vec![1, 5, 30, 0, 20, 31];
    let test1 = vec![
        vec![1,4,7,11,15],
        vec![2,5,8,12,19],
        vec![3,6,9,16,22],
        vec![10,13,14,17,24],
        vec![18,21,23,26,30]
    ];

    for test in test_set1 {
        println!("{} -> {}", test, Solution::search_matrix(test1.clone(), test));
    }
}
