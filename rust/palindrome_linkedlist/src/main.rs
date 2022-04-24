use my_utils::my_utils::ListNode;

struct Solution {}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        true
    }
}

fn main() {
    println!("Is linked list a palindrome?");

    let mut test_set = Vec::new();

    test_set.push(vec![0,1,2,3]);
    test_set.push(vec![0,1,2,2,1]);
    test_set.push(vec![0,1,2,1]);

    for test in test_set {
        let llist = ListNode::from_vec(&test).next;
        println!("LinkedList = {:?}, palindrome={}", &test[1..], Solution::is_palindrome(llist));
    }
}
