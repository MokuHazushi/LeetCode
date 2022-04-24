pub mod my_utils {

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>
    }

    impl ListNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            ListNode {
                next: None,
                val
            }
        }

        pub fn from_vec(list: &Vec<i32>) -> Option<Box<ListNode>> {
            if list.is_empty() {
                return None;
            }

            let mut head = ListNode::new(list[0]);
            let mut iter = &mut head;

            for n in list {
                iter.next = Some(Box::new(ListNode::new(*n)));
                iter = iter.next.as_mut().unwrap().as_mut();
            }

            head.next
        }

        pub fn to_vec(llist: &Option<Box<ListNode>>) -> Vec<i32> {
            let mut list = Vec::new();

            let mut iter = llist.as_ref();
            while iter != None {
                if let Some(ref node) = iter {
                    list.push(node.as_ref().val);
                    iter = node.next.as_ref();
                }
            }

            list
        }

    }

}

#[cfg(test)]
mod tests {

    use crate::my_utils::*;

    #[test]
    fn build_linked_list() {
        let source = vec![1,2,3];
        let llist = ListNode::from_vec(&source);
        let result = ListNode::to_vec(&llist);

        for i in 0..source.len() {
            assert_eq!(source[i], result[i]);
        }
    }

    #[test]
    fn build_empty_linked_list() {
        let source = Vec::new();
        let llist = ListNode::from_vec(&source);
        let result = ListNode::to_vec(&llist);

        assert_eq!(llist, None);
        assert!(result.is_empty());
        
    }
}
