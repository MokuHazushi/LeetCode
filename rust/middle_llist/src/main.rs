// https://leetcode.com/problems/middle-of-the-linked-list/

use my_utils::my_utils::ListNode;

struct Solution {}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut iter, mut iter_slow) = (head.clone(), head.clone());
        let mut count = 0;
        while let Some(node) = iter {
            iter = node.next;
            
            if count == 1 {
                if let Some(slow_node) = iter_slow {
                    iter_slow = slow_node.next;
                    count = 0;
                }
            }
            else {
                count += 1;
            }
        }
        iter_slow
    }
}

fn main() {
    println!("Find middle of linkedlist");

    let mut test_set = Vec::new();

    test_set.push(vec![1,2,3]);
    test_set.push(vec![1,2,3,4]);
    test_set.push(vec![1]);
    test_set.push(vec![1,2]);

    for test in test_set {
        let llist = ListNode::from_vec(&test);
        let middle_llist = Solution::middle_node(llist);
        println!("LinkedList={:?}, middle={:?}", test, ListNode::to_vec(&middle_llist));
    }
}
