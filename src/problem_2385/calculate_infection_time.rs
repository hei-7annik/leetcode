use std::cell::{RefCell};
use std::rc::Rc;
use std::cmp::{max};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: u32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: u32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}



pub fn calculate_infection_time_naive(root: Option<Rc<RefCell<TreeNode>>>, start: u32) -> u32 {
    depth_mapping(root, start).max_path_len
}

#[derive(Debug)]
pub struct Results {
    max_path_len: u32,
    path_to_infected_len: Option<u32>,
}

/* Options
1.  Infection starts on one and travels to the other side of the tree
    longest path:   infected node -> ancestor -> ... -> root -> ... -> child

2.  Infection starts on one and travels to the other side, which gets fully infected before completing
    the side it started on.
    longest path:   infected node -> ancestor -> ... -> ancestor -> .. -> child

3.  Infection takes longer travels to one of the subtrees of its node then through the rest of the tree
    longest path:   infected node -> child -> ... -> child
*/
pub fn depth_mapping(root: Option<Rc<RefCell<TreeNode>>>, target: u32) -> Results {
    let mut result = Results { max_path_len: 0, path_to_infected_len: None };

    if let Some(node) = root {
        let left_subtree = depth_mapping(node.borrow().left.clone(), target);
        let right_subtree = depth_mapping(node.borrow().right.clone(), target);

        result.max_path_len = max(left_subtree.max_path_len, right_subtree.max_path_len);

        if node.borrow().val == target {
            result.path_to_infected_len = Some(0);
        }
        // Option 1 & 2
        else if let Some(length) = left_subtree.path_to_infected_len {
            result.path_to_infected_len = Some(length + 1);
            result.max_path_len = max(result.path_to_infected_len.unwrap() + right_subtree.max_path_len,
                                      result.max_path_len);
        }
        // Option 1 & 2
        else if let Some(length) = right_subtree.path_to_infected_len {
            result.path_to_infected_len = Some(length + 1);
            result.max_path_len = max(result.path_to_infected_len.unwrap() + left_subtree.max_path_len,
                                      result.max_path_len);
        }
        // Option 3
        else {
            result.max_path_len += 1
        }
    }
    result
}