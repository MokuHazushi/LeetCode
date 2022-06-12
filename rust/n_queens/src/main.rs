// https://leetcode.com/problems/n-queens/

struct Solution {}

use std::collections::VecDeque;

struct BfsNode {
    row_pos: usize,
    board: Vec<Vec<char>>, // 'Q' = queen, 'X' = attacked by a queen, '.' = free
}

impl BfsNode {
    pub fn new(row_pos: usize, board: Vec<Vec<char>>) -> Self {
        BfsNode {
            row_pos: row_pos,
            board: board,
        }
    }
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n_factorial = {
            let mut factorial = 1;
            let mut n = n;
            while n > 0 {
                factorial *= n;
                n -= 1;
            }
            factorial as usize
        };
        let mut bfs_search = VecDeque::with_capacity(n_factorial);
        let mut solutions = Vec::new();
        
        bfs_search.push_back(BfsNode::new(0, vec![vec!['.'; n as usize]; n as usize]));

        while !bfs_search.is_empty() {
            let node = bfs_search.pop_front().unwrap();

            if node.row_pos == n as usize {
                solutions.push(node.board);
                continue;
            }

            for i in 0..n {
                if node.board[node.row_pos][i as usize] != '.' {
                    continue;
                }
                let mut new_board = node.board.clone();
                Solution::update_board((node.row_pos, i as usize), &mut new_board);
                bfs_search.push_back(BfsNode::new(node.row_pos+1, new_board));
            }
        }

        Solution::convert_solution(solutions)
    }

    fn update_board(queen_pos: (usize, usize), board: &mut Vec<Vec<char>>) {
        board[queen_pos.0][queen_pos.1] = 'Q';
        let queen_col = queen_pos.1 as i32;
        let (mut left, mut right) = (queen_col-1, queen_col+1);
        let mut row = queen_pos.0 + 1;

        while row < board.len() {
            if left >= 0 && board[row][left as usize] == '.' {
                board[row][left as usize] = 'X';
            }
            if right < board.len() as i32 && board[row][right as usize] == '.' {
                board[row][right as usize] = 'X';
            }
            board[row][queen_pos.1] = 'X';

            row += 1;
            left -= 1;
            right += 1;
        }
    }

    fn convert_solution(solutions: Vec<Vec<Vec<char>>>) -> Vec<Vec<String>> {
        let mut final_solution = Vec::new();
        for solution in solutions {
            let mut ans = Vec::new();
            for char_vec in solution {
                ans.push(char_vec.iter().fold(String::from(""), |mut acc, c| {
                    if c == &'Q' {
                        acc.push(*c);
                    }
                    else {
                        acc.push('.');
                    }
                    acc
                }));
            }
            final_solution.push(ans);
        }
        final_solution
    }
}

fn main() {
    println!("Solve N-Queens problem");

    let test_set = vec![4];

    for test in test_set {
        println!("n={}\n-> {:?}", test, Solution::solve_n_queens(test));
    }
}
