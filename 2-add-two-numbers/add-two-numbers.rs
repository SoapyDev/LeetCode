// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;
        let mut p1 = l1;
        let mut p2 = l2;
        let mut carry = 0;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let val1 = p1.as_ref().map_or(0, |n| n.val);
            let val2 = p2.as_ref().map_or(0, |n| n.val);
            let sum = val1 + val2 + carry;

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            p1 = p1.and_then(|n| n.next);
            p2 = p2.and_then(|n| n.next);
        }

        dummy_head.next
    }
}
