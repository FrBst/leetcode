use std::{cell::RefCell, rc::Rc};

use crate::common::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::good_nodes_recursive(root, i32::MIN)
    }

    fn good_nodes_recursive(root: Option<Rc<RefCell<TreeNode>>>, max_before: i32) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let mut count = 0;
                let current_max = root.borrow().val.max(max_before);
                count += Self::good_nodes_recursive(root.borrow().left.clone(), current_max);
                count += Self::good_nodes_recursive(root.borrow().right.clone(), current_max);

                if root.borrow().val >= max_before {
                    count += 1;
                }

                count
            }
        }
    }

    fn good_nodes_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        let mut queue = vec![(root.unwrap(),i32::MIN)];
        let mut count = 0;

        while !queue.is_empty() {
            let (node, before_max) = queue.pop().unwrap();
            let current_max = before_max.max(node.borrow().val);

            match node.borrow().left.clone() {
                None => (),
                Some(node) => queue.push((node,current_max))
            }

            match node.borrow().right.clone() {
                None => (),
                Some(node) => queue.push((node,current_max))
            }

            if node.borrow().val >= before_max {
                count += 1;
            }
        }

        count
    }
}