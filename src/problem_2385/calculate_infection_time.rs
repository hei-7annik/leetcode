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

/// Simulate a spreading infection by searching for the longest
/// path in an undirected graph.
///
/// # Example
///
///             (0)
///           //   \\
///         (1)     (2)
///       //       /   \\
///     (3)      (4)   (5)
///                 \
///                  (6)
///
/// ```rust
/// debug_assert_eq!(calculate_infection_time(root, 5), 4);
/// ```
///
/// # Cases
/// 1. Longest path from the infected node goes up and then down
/// 2. Longest path from the infected node goes down
///
/// # Method
/// 1. Recursively traverse the tree depth first
/// 2. Increase distance `l` to leaf by 1
/// 3. Increase distance `i` to infected node if it is part of left/right subtree
/// 4. Set path length `p = l + i`
/// 5. Compare `p` to `max_p` and update `max_p` if necessary
///
pub fn calculate_infection_time(root: Option<Rc<RefCell<TreeNode>>>, start: u32) -> u32 {
    depth_mapping(root, start).max_path_len
}

#[derive(Debug)]
pub struct Results {
    max_path_len: u32,
    path_to_infected_len: Option<u32>,
}

pub fn depth_mapping(root: Option<Rc<RefCell<TreeNode>>>, target: u32) -> Results {
    let mut result = Results { max_path_len: 0, path_to_infected_len: None };

    if let Some(node) = root {
        let left_subtree = depth_mapping(node.borrow().left.clone(), target);
        let right_subtree = depth_mapping(node.borrow().right.clone(), target);

        result.max_path_len = max(left_subtree.max_path_len, right_subtree.max_path_len);

        if node.borrow().val == target {
            result.path_to_infected_len = Some(0);
        }
        // Path length calculation for Case 1
        else if let Some(length) = left_subtree.path_to_infected_len {
            result.path_to_infected_len = Some(length + 1);
            result.max_path_len = max(result.path_to_infected_len.unwrap() + right_subtree.max_path_len,
                                      result.max_path_len);
        }
        else if let Some(length) = right_subtree.path_to_infected_len {
            result.path_to_infected_len = Some(length + 1);
            result.max_path_len = max(result.path_to_infected_len.unwrap() + left_subtree.max_path_len,
                                      result.max_path_len);
        }
        // Case 2
        else {
            result.max_path_len += 1
        }
    }
    result
}