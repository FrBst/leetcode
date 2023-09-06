use std::cell::RefCell;
use std::rc::Rc;

use crate::common::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::get_depth_and_diameter(root).1
    }
    
    fn get_depth_and_diameter(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => (0, 0),
            Some(root) => {
                let left = Self::get_depth_and_diameter(root.borrow().left.clone());
                let right = Self::get_depth_and_diameter(root.borrow().right.clone());
    
                let depth = i32::max(left.0, right.0) + 1;
                let diameter = i32::max(left.0 + right.0, i32::max(left.1, right.1));
    
                (depth, diameter)
            }
        }
    }
}