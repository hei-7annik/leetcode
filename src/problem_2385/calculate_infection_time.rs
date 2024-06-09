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
    depth_mapping(root, start, 0).longest_path
}

#[derive(Debug)]
pub struct Results {
    longest_path: i32,
    infected: Option<i32>,
}

pub fn depth_mapping(root: Option<Rc<RefCell<TreeNode>>>, target: i32, current_depth: i32) -> Results {
    let mut result = Results {longest_path: current_depth - 1, infected: None };

    // walk through
    if let Some(node) = root {
        let result_left = depth_mapping(node.borrow().left.clone(), target, current_depth + 1);
        let result_right = depth_mapping(node.borrow().right.clone(), target, current_depth + 1);

        result.longest_path = max(result_left.longest_path, result_right.longest_path);

        // Option 1 & 2
        if let Some(infected) = result_left.infected {
            let path_infected_to_right_leaf = result_left.infected.unwrap() - 2* current_depth + result_right.longest_path;
            result.longest_path = max(path_infected_to_right_leaf, result_left.longest_path);
            result.infected = Some(infected);
        }
        else if let Some(infected) = result_right.infected {
            let path_infected_to_left_leaf = result_right.infected.unwrap() - 2* current_depth + result_left.longest_path;
            result.longest_path = max(path_infected_to_left_leaf, result_right.longest_path);
            result.infected = Some(infected);
        }

        // Option 3
        if node.borrow().val == target {
            result.infected = Some(current_depth);
        }
    }
    result
}