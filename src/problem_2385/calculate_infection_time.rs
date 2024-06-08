use std::cell::{RefCell};
use std::rc::Rc;
use std::cmp::{max};

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

// Options
// 1. The path from infected_node to furthest_leaf includes root
// 2. The path root -> infected_node and root -> furthest_leaf is the same for the first n steps
// 3. The path root -> infected_node is part of root -> furthest_leaf

pub fn calculate_infection_time_naive(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    let depth_values = depth_mapping(root, start, 0);

    // option 1 and 2
    if let Some(depth_common_ancestor) = depth_values.common_ancestor {
        return depth_values.infected.unwrap() - depth_common_ancestor + depth_values.depth
    }
    // option 3
    else {
        return depth_values.depth - depth_values.infected.unwrap()
    }
}

#[derive(Debug)]
pub struct Results {
    depth: i32,
    infected: Option<i32>,
    common_ancestor: Option<i32>
}



pub fn depth_mapping(root: Option<Rc<RefCell<TreeNode>>>, target: i32, depth: i32) -> Results {
    let mut result = Results {depth: depth - 1, infected: None, common_ancestor: None};

    // walk through
    if let Some(node) = root {
        let result_left = depth_mapping(node.borrow().left.clone(), target, depth + 1);
        let result_right = depth_mapping(node.borrow().right.clone(), target, depth + 1);

        //
        result.infected = result_left.infected.or(result_right.infected);
        result.depth = max(result_left.depth, result_right.depth);

        if node.borrow().val == target {
            result.infected = Some(depth);
        }

        if (result_left.infected.or(result_right.infected) != None) && (result_left.depth != result_right.depth) {
            result.common_ancestor = Some(depth);
        }
    }
    result
}