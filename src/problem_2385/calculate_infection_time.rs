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
    depth_mapping(root, start, 0).longest_path
}

#[derive(Debug)]
pub struct Results {
    longest_path: u32,
    infected: Option<u32>,
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
pub fn depth_mapping(root: Option<Rc<RefCell<TreeNode>>>, target: u32, current_depth: u32) -> Results {
    if let Some(node) = root {
        let left_subtree = depth_mapping(node.borrow().left.clone(), target, current_depth + 1);
        let right_subtree = depth_mapping(node.borrow().right.clone(), target, current_depth + 1);

        // Set baseline for Option 3
        if node.borrow().val == target && left_subtree.infected.is_none() && right_subtree.infected.is_none() {
            let infected = Some(current_depth);
            let longest_path = max(left_subtree.longest_path, right_subtree.longest_path);

            return Results{longest_path, infected}
        }
        else {
            let infected = left_subtree.infected.or(right_subtree.infected);
            // Determine which side of the tree is infected
            let longest_path = match infected {
                // Option 1 & 2
                Some(infected_node_depth) if left_subtree.infected.is_some() =>
                    max(infected_node_depth - current_depth + right_subtree.longest_path, left_subtree.longest_path),
                Some(infected_node_depth) if right_subtree.infected.is_some() =>
                    max(infected_node_depth - current_depth + left_subtree.longest_path, right_subtree.longest_path),
                // Option 3
                _ => max(left_subtree.longest_path, right_subtree.longest_path) + 1
            };
            return Results{longest_path, infected}
        };
    }
    Results {longest_path: 0, infected: None }
}