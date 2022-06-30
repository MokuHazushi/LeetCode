// https://leetcode.com/problems/furthest-building-you-can-reach/

struct Solution {}

use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut ladders = ladders;
        let mut bricks = bricks;
        let mut index = 1;
        let mut max_heights = BinaryHeap::new();

        loop {
            while index < heights.len() {
                let height_diff = heights[index] - heights[index-1];

                if height_diff <= 0 {
                    index += 1;
                    continue;
                }

                if height_diff <= bricks {
                    bricks -= height_diff;
                    index += 1;
                    max_heights.push(height_diff);
                    continue;
                }
                break;
            }

            if index == heights.len() || ladders == 0 {
                return index as i32 - 1;
            }

            ladders -= 1;
            if let Some(max_height_diff) = max_heights.peek() {
                if *max_height_diff > heights[index] - heights[index-1] {
                    bricks += max_height_diff;
                    max_heights.pop();
                }
                else {
                    index += 1;
                }
            }
            else {
                index += 1;
            }
        }
    }
}

struct Test {
    heights: Vec<i32>,
    bricks: i32,
    ladders: i32,
}

impl Test {
    pub fn new(heights: Vec<i32>, bricks: i32, ladders: i32) -> Self {
        Test {
            heights: heights,
            bricks: bricks,
            ladders: ladders,
        }
    }
}

fn main() {
    println!("Find furthest building you can read");
    let mut test_set = Vec::new();

    test_set.push(Test::new(vec![4,2,7,6,9,14,12], 5, 1));
    test_set.push(Test::new(vec![4,12,2,7,3,18,20,3,19], 10, 2));

    for test in test_set {
        println!("heights={:?}, bricks={}, ladders={}", test.heights, test.bricks, test.ladders);
        println!("\t-> {}", Solution::furthest_building(test.heights, test.bricks, test.ladders));
    }
}
