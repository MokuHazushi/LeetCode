// https://leetcode.com/problems/is-graph-bipartite/

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut checked_nodes = vec![false; graph.len()];

        if !Solution::is_connected(&graph) {
            return false;
        }

        for i in 0..checked_nodes.len() {
            if checked_nodes[i] {
                continue;
            }

            if let Some(cycle) = Solution::find_cycle(&graph, i as i32) {
                if cycle.len() % 2 == 1 {
                    return false;
                }
                for node in cycle {
                    checked_nodes[node as usize] = true;
                }
            }
        }
        true
    }

    pub fn is_connected(graph: &Vec<Vec<i32>>) -> bool {
        let mut stack = Vec::new();
        let mut visited = HashMap::new();

        stack.push(0);
        visited.insert(0, true);

        while let Some(next_node) = stack.pop() {
            visited.insert(next_node, true);
            for neighbor in &graph[next_node as usize] {
                if visited.contains_key(neighbor) {
                    continue;
                }
                stack.push(*neighbor);
            }
        }

        visited.keys().len() == graph.len()
    }

    pub fn find_cycle(graph: &Vec<Vec<i32>>, root: i32) -> Option<Vec<i32>> {
        let mut stack = Vec::new();
        let mut visited = HashMap::new();
        let mut parents = HashMap::new();

        parents.insert(root, -1);
        stack.push(root);

        while let Some(next_node) = stack.pop() {
            
            visited.insert(next_node, true);
            for neighbor in &graph[next_node as usize] {

                if *neighbor == root {
                    if *parents.get(&next_node).unwrap() == root {
                        continue;
                    }
                    parents.insert(root, next_node);
                    return Some(Solution::reconstruct_path(root, &parents));
                }

                if visited.contains_key(neighbor) {
                    continue;
                }

                parents.insert(*neighbor, next_node);
                stack.push(*neighbor);
            }
        }
        None
    }

    fn reconstruct_path(root: i32, parents: &HashMap<i32, i32>) -> Vec<i32> {
        let mut path = vec![root];
        let mut iter = root;

        while let Some(parent) = parents.get(&iter) {
            if *parent == root {
                break;
            }
            path.push(*parent);
            iter = *parent;
        } 

        path
    }
}

fn main() {
    println!("Is graph bipartite?");
    let mut test_set = Vec::new();
/*
    test_set.push(vec![
        vec![1,2], 
        vec![0,2], 
        vec![0,1]
    ]);

    test_set.push(vec![
        vec![1,3], 
        vec![0,2], 
        vec![1,3],
        vec![0,2]
    ]);
    test_set.push(vec![
        vec![1,4], 
        vec![0,2,3], 
        vec![1,3],
        vec![1,2,4],
        vec![0,3]
    ]);
    test_set.push(vec![
        vec![2],
        vec![],
        vec![0]
    ]);

    test_set.push(vec![
        vec![1,2],
        vec![0,3],
        vec![0,3],
        vec![1,2]
    ]);
*/

    test_set.push(vec![
        vec![1,2,3],
        vec![0,2],
        vec![0,1,3],
        vec![0,2]
    ]);

    for test in test_set {
        println!("{:?}\n\t-> {}", test.clone(), Solution::is_bipartite(test));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cycle() {
        let graph1 = vec![
            vec![1,2], 
            vec![0,2], 
            vec![0,1]
        ];

        let graph2 = vec![
            vec![1,3], 
            vec![0,2], 
            vec![1,3],
            vec![0,2]
        ];

        let graph3 = vec![
            vec![1,4], 
            vec![0,2,3], 
            vec![1,3],
            vec![1,2,4],
            vec![0,3]
        ];
        
        let mut cycle1 = Solution::find_cycle(&graph1, 0).unwrap();
        cycle1.sort();
        assert_eq!(cycle1, vec![0,1,2]);

        let mut cycle2 = Solution::find_cycle(&graph2, 0).unwrap();
        cycle2.sort();
        assert_eq!(cycle2, vec![0,1,2,3]);

        let mut cycle3 = Solution::find_cycle(&graph3, 2).unwrap();
        cycle3.sort();
        assert_eq!(cycle3, vec![0,1,2,3,4]);
    }

    #[test]
    fn test_is_connected() {
        let graph1 = vec![
            vec![1,2], 
            vec![0,2], 
            vec![0,1]
        ];

        let graph2 = vec![
            vec![2],
            vec![],
            vec![0]
        ];

        let graph3 = vec![
            vec![4],
            vec![],
            vec![4],
            vec![4],
            vec![0,2,3]
        ];

        assert_eq!(Solution::is_connected(&graph1), true);
        assert_eq!(Solution::is_connected(&graph2), false);
        assert_eq!(Solution::is_connected(&graph3), false);
    }

}
