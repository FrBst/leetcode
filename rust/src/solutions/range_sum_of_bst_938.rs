use std::rc::Rc;
use std::cell::RefCell;

use crate::common::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let rc = root.unwrap();
        let node = rc.borrow();
        let mut sum = 0;

        if node.val > low && node.left.is_some() {
            sum += Self::range_sum_bst(node.left.clone(), low, high);
        }
        if node.val >= low && node.val <= high {
            sum += node.val;
        } 
        if node.val < high && node.right.is_some() {
            sum += Self::range_sum_bst(node.right.clone(), low, high);
        }

        sum
    }
}