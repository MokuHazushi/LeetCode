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

        pub fn from_vec(list: &Vec<i32>) -> Self {
            if list.is_empty() {
                panic!("Expected non-empty list");
            }

            let mut head = ListNode::new(list[0]);
            let mut iter = &mut head;

            for i in 1..list.len() {
                iter.next = Some(Box::new(ListNode::new(list[i])));
                iter = iter.next.as_mut().unwrap().as_mut();
            }

            head
        }

        pub fn to_vec(&self) -> Vec<i32> {
            let mut list = vec![self.val];

            let mut iter = self.next.as_ref();
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
        let list = vec![1,2,3];
        let llist = ListNode::from_vec(&list);
        let mut i = 0;
        let mut iter = &llist;

        loop {
            assert_eq!(list[i], iter.val);
            
            i += 1;
            match &iter.next {
                Some(node) => iter = node.as_ref(),
                None => break,
            }
        }

    }

    #[test]
    fn recover_linked_list() {
        let list = vec![1,2,3];
        let llist = ListNode::from_vec(&list);
        let recovered_list = llist.to_vec();

        for i in 0..recovered_list.len() {
            assert_eq!(list[i], recovered_list[i]);
        }


    }
}
