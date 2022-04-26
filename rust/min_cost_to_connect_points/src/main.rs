// https://leetcode.com/problems/min-cost-to-connect-all-points/

use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Solution {}

struct SearchElement {
    cost: i32,
    point: Vec<i32>,
}

impl Solution {
    pub fn min_cost_connect_points(mut points: Vec<Vec<i32>>) -> i32 {
        let nb_points = points.len();
        let mut total_cost = 0;
        let mut search_heap = BinaryHeap::new();
        let mut visisted_nodes = Vec::new();
        let first_point = points.remove(0);

        Solution::populate_heap(&first_point, &points, &mut search_heap);
        visisted_nodes.push(first_point);

        while visisted_nodes.len() < nb_points {
            if let Some(next_point) = search_heap.pop() {
                if visisted_nodes.contains(&next_point.point) {
                    continue;
                }
                total_cost += next_point.cost;
                points.retain(|p| *p != next_point.point);
                visisted_nodes.push(next_point.point.clone());
                Solution::populate_heap(&next_point.point, &points, &mut search_heap);
            }
        }

        total_cost
    }

    fn populate_heap(point: &Vec<i32>, points: &Vec<Vec<i32>>, heap: &mut BinaryHeap<SearchElement>) {
        for other_point in points {
            let dist = Solution::manhattan_distance(&point, &other_point);
            heap.push(SearchElement::new(dist, other_point.clone()));
        }
    }

    fn manhattan_distance(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
        (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
    }

}

impl SearchElement {
    fn new(cost: i32, point: Vec<i32>) -> Self {
        SearchElement {
            cost: cost,
            point: point,
        }
    }
}

impl Ord for SearchElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cost.cmp(&other.cost) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
        }
    }
}

impl Eq for SearchElement {}

impl PartialOrd for SearchElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.cost.cmp(&other.cost) {
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => Some(Ordering::Equal),
        }
    }
}

impl PartialEq for SearchElement {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn main() {
    println!("Find minimal cost to connect all points with a single path.");
    let mut test_set = Vec::new();

    test_set.push(vec![vec![0,0], vec![2,2], vec![3,10], vec![5,2], vec![7,0]]);
    test_set.push(vec![vec![3,12], vec![-2,5], vec![-4, 1]]);
    test_set.push(vec![vec![0,0], vec![1,0], vec![1,1], vec![0,1]]);
    test_set.push(vec![vec![0,0], vec![0,1], vec![5,0], vec![5,1]]);
    test_set.push(vec![vec![-8,14], vec![16,-18], vec![-19,-13], vec![-18,19], vec![20,20], vec![13,-20], vec![-15,9], vec![-4,-8]]);

    for test in test_set {
        println!("Points={:?} -> {}", test.clone(), Solution::min_cost_connect_points(test));
    }
}
