use std::cell::{RefCell};
use std::rc::Rc;
use std::cmp::max;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

fn depth(root: Option<Rc<RefCell<TreeNode>>>, tree_depth: i32) -> i32 {
    if let Some(current) = root {
        let node = current.borrow();

        let mut depth_left_arm = 0;
        if let left_child = node.left.to_owned() {
            depth_left_arm = depth(left_child, tree_depth + 1);
        }
        let mut depth_right_arm = 0;
        if let right_child = node.right.to_owned() {
            depth_right_arm = depth(right_child, tree_depth + 1);
        }

        return max(depth_left_arm, depth_right_arm)
    };
    tree_depth - 1
}

fn depth_of(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> i32 {
    let mut depth = 0;
    if let Some(node) = root {
        let mut current_node = node.borrow().clone();

        while current_node.left != None || current_node.right != None {
            match current_node.val {
                n if target > n     => current_node = current_node.right.clone().unwrap().borrow().clone(),
                n if target < n     => current_node = current_node.left.clone().unwrap().borrow().clone(),
                _                        => break,
            }
            depth += 1;
        }
    }
    depth
}

pub fn calculate_infection_time_naive(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    depth(root.clone(), 0) + depth_of(root, start)
}