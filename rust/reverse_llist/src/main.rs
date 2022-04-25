// https://leetcode.com/problems/reverse-linked-list/

use my_utils::my_utils::ListNode;

struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;
        while let Some(mut current_box) = current {
            current = current_box.next.take();
            current_box.next = prev;
            prev = Some(current_box);
        }
        prev
    }
}

fn main() {
    println!("Reverse linked list");
    let mut test_set = Vec::new();

    test_set.push(vec![1,2,3]);
    test_set.push(vec![3]);


    for test in test_set {
        let llist = ListNode::from_vec(&test);
        let reversed_list = ListNode::to_vec(&Solution::reverse_list(llist));

        println!("LinkedList={:?}", test);
        println!("\t-> {:?}", reversed_list);
    }
}
