struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let (mut i, mut j) = (0, letters.len()-1);

        while i < j {
            let mid = (i+j)/2;

            if letters[mid] <= target && letters[mid+1] > target {
                return letters[mid+1];
            }
            if target >= letters[mid] {
                i = mid+1;
            }
            else {
                j = mid;
            }
        }
        letters[0]
    }
}

fn main() {
    let test_cases = vec![
        (vec!['c', 'f', 'j'], 'a'),
        (vec!['c', 'f', 'j'], 'c'),
        (vec!['x', 'x', 'y', 'y'], 'z'),
        (vec!['e', 'e', 'e', 'e', 'e', 'e', 'n', 'n', 'n', 'n'], 'e')
    ];

    for test in &test_cases {
        println!("{:?} -> {}", test, Solution::next_greatest_letter(test.0.clone(), test.1));
    }
}
