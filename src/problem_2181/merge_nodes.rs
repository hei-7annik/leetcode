use std::boxed::Box;

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
}

pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode::new(0)))
}