// https://leetcode.com/problems/permutations-ii/

struct Solution {}

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct BFSElement {
    nb_time_visited: HashMap<i32, i32>,
    permutation: Vec<i32>,
}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = Vec::new();
        let mut elements_freq: HashMap<i32, i32> = HashMap::new();

        for n in &nums {
            let freq = elements_freq.entry(*n).or_insert(0);
            *freq += 1;
        }

        //println!("elements_freq={:?}", elements_freq);

        for unique_el in elements_freq.keys() {
            permutations.append(&mut Solution::find_all_bfs(*unique_el, &elements_freq, nums.len()));
        }
        permutations
    }

    fn find_all_bfs(num: i32, elements_freq: &HashMap<i32, i32>, nums_size: usize) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut permutations = Vec::new();
        let mut nb_time_visited: HashMap<i32, i32> = HashMap::new();

        nb_time_visited.insert(num, 1);
        queue.push_front(BFSElement::new(nb_time_visited, vec![num]));

        while let Some(mut bfs_el) = queue.pop_back() {
            //println!("next queue el={:?}", bfs_el);

            if bfs_el.permutation.len() == nums_size {
                permutations.push(bfs_el.permutation.clone());
                continue;
            }

            for neighbor in elements_freq.keys() {
                let cur_freq = bfs_el.nb_time_visited.entry(*neighbor).or_insert(0);
                if cur_freq == elements_freq.get(&neighbor).unwrap() {
                    continue;
                }

                let mut new_el = BFSElement::new(bfs_el.nb_time_visited.clone(), bfs_el.permutation.clone());
                new_el.nb_time_visited.insert(*neighbor, new_el.nb_time_visited.get(&neighbor).unwrap()+1);
                new_el.permutation.push(*neighbor);
                queue.push_front(new_el);
            }
        }
        permutations
    }
}

impl BFSElement {
    fn new(nb_time_visited: HashMap<i32, i32>, permutation: Vec<i32>) -> Self {
        BFSElement {
            nb_time_visited: nb_time_visited,
            permutation: permutation,
        }
    }
}

fn main() {
    println!("Find all unique permutation");
    let mut test_set = Vec::new();

    test_set.push(vec![1,1,2]);
    test_set.push(vec![1,2,3]);

    for test in test_set {
        println!("nums={:?}", test);
        println!("-> {:?}", Solution::permute_unique(test));
    }
}
