use std::rc::Rc;
use std::cell::RefCell;

use crate::common::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        Self::find_max(root.as_ref(), &mut res, 0);

        res
    }

    fn find_max(root: Option<&Rc<RefCell<TreeNode>>>, max: &mut Vec<i32>, lvl: usize) {
        match root {
            None => return,
            Some(node) => {
                if (max.len() <= lvl) { max.push(i32::MIN); }
                max[lvl] = max[lvl].max(node.borrow().val);
                Self::find_max(node.borrow().left.as_ref(), max, lvl + 1);
                Self::find_max(node.borrow().right.as_ref(), max, lvl + 1);
            }
        }
    }
}