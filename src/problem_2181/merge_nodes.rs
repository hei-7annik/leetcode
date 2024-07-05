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

pub fn list_from(values: &[i32]) -> Option<Box<ListNode>> {
  let mut current= None;

  for i in (0..values.len()).rev() {
    current = Some(Box::new(ListNode{ val: values[i], next: current }));
  }
  current
}

pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut list = Vec::new();
  let mut acc = 0;

  while let Some(node) = head {
    if node.val == 0 {
      list.push(acc);
      acc = 0;
    }
    else {
      acc += node.val;
    }
    head = node.next;
  }
  list_from(&list[1..])
}