// https://leetcode.com/problems/spiral-matrix-ii/

struct Solution {}

enum Direction { LEFT, RIGHT, UP, DOWN }

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut mat : Vec<Vec<i32>> = Vec::new();

        for _i in 0..n {
            let mut tmp : Vec<i32> = Vec::new();
            for _j in 0..n {
                tmp.push(1);
            }
            mat.push(tmp);
        }

        if n == 1 {
            return mat
        }

        let mut i: usize; // rows
        let mut j: usize; // colums
        let mut width: (usize, usize) = (0, n as usize);
        let mut height: (usize, usize) = (0, n as usize);
        let mut nb_filled_els = 1;
        let mut direction = Direction::RIGHT;

        while nb_filled_els <= n*n {
            match direction {
                Direction::RIGHT => {
                    j = height.0;
                    for i in width.0..width.1 {
                        mat[j][i] = nb_filled_els;
                        nb_filled_els += 1;
                    }
                    height.0 += 1;
                },
                Direction::DOWN => {
                    i = width.1 - 1;
                    for j in height.0..height.1 {
                        mat[j][i] = nb_filled_els;
                        nb_filled_els += 1;
                    }
                    width.1 -= 1;
                },
                Direction::LEFT => {
                    j = height.1 - 1;
                    for i in (width.0..width.1).rev() {
                        mat[j][i] = nb_filled_els;
                        nb_filled_els += 1;
                    }
                    height.1 -= 1;
                },
                Direction::UP => {
                    i = width.0;
                    for j in (height.0..height.1).rev() {
                        mat[j][i] = nb_filled_els;
                        nb_filled_els += 1;
                    }
                    width.0 += 1;
                },
            }
            direction = Solution::next_direction(direction);
        }

        mat
    }

    fn next_direction(direction: Direction) -> Direction {
        match direction {
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::UP => Direction::RIGHT,
        }
    }
}

fn main() {
    println!("Creating a spiral matrice nxn");
    let test_set = vec![1,2,3];

    for n in test_set {
        println!("n={}", n);
        println!("{:?}", Solution::generate_matrix(n));
    }
}
