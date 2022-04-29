// https://leetcode.com/problems/path-with-minimum-effort/

struct Solution {}

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp;

struct Node {
    coords: (usize, usize),
    cost: i32,
}

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let dims = (heights.len(), heights[0].len());
        let goal_node = (dims.0-1, dims.1-1);
        let mut heap = BinaryHeap::new();
        let mut costs = vec![vec![i32::MIN; dims.1]; dims.0];

        heap.push(Node::new((0,0), 0));

        while let Some(next_node) = heap.pop() {
            //println!("next_node = {:?}", next_node.coords);
            if next_node.coords == goal_node {
                return -next_node.cost;
            }

            for coords in Solution::get_neighbors(next_node.coords, dims) {
                let cost = cmp::min(-Solution::abs_height(coords, next_node.coords, &heights), next_node.cost);
                let node = Node::new(coords, cost);
                if node.cost > costs[coords.0][coords.1] {
                    heap.push(node);
                    costs[coords.0][coords.1] = cost;
                }
            }
        }
        unreachable!();
    }

    pub fn get_neighbors(coords: (usize, usize), dims: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (i, j) = (coords.0, coords.1);

        if i < dims.0-1 {
            neighbors.push((i+1, j));
        }
        if i > 0 {
            neighbors.push((i-1, j));
        }
        if j < dims.1-1 {
            neighbors.push((i, j+1));
        }
        if j > 0 {
            neighbors.push((i, j-1));
        }

        neighbors
    }

    fn abs_height(p1: (usize, usize), p2: (usize, usize), heights: &Vec<Vec<i32>>) -> i32 {
        (heights[p1.0][p1.1] - heights[p2.0][p2.1]).abs()
    }
}

impl Node {
    fn new(coords: (usize, usize), cost: i32) -> Self {
        Node {
            coords: coords,
            cost: cost,
        }
    }
}

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cost.cmp(&other.cost))
    }
}

fn main() {
    println!("Find path with minimum effort");
    let mut test_set = Vec::new();

    
    test_set.push(vec![
        vec![1,2,2],
        vec![3,8,2],
        vec![5,3,5]
    ]);
    
    test_set.push(vec![
        vec![1,2,3],
        vec![3,8,4],
        vec![5,3,5]
    ]);
    
    test_set.push(vec![
        vec![1,2,1,1,1],
        vec![1,2,1,2,1],
        vec![1,2,1,2,1],
        vec![1,2,1,2,1],
        vec![1,1,1,2,1]
    ]);
    
    test_set.push(vec![
        vec![1,1,1],
        vec![3,3,10],
        vec![3,1,1]
    ]);
    
    for test in test_set {
        println!("{:?}\n\t->{}", test.clone(), Solution::minimum_effort_path(test));
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbors() {
        let dims = (3,3);

        assert_eq!(Solution::get_neighbors((1,1), dims).len(), 4);
        // corners
        assert_eq!(Solution::get_neighbors((0,0), dims).len(), 2);
        assert_eq!(Solution::get_neighbors((0,2), dims).len(), 2);
        assert_eq!(Solution::get_neighbors((2,2), dims).len(), 2);
        assert_eq!(Solution::get_neighbors((2,0), dims).len(), 2);

        // edges
        assert_eq!(Solution::get_neighbors((0,1), dims).len(), 3);
        assert_eq!(Solution::get_neighbors((1,0), dims).len(), 3);
        assert_eq!(Solution::get_neighbors((2,1), dims).len(), 3);
        assert_eq!(Solution::get_neighbors((1,2), dims).len(), 3);
    }

}
