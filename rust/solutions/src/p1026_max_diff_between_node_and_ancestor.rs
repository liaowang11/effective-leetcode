// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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
            right: None,
        }
    }
}

struct Solution {}
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let max = Rc::new(RefCell::new(i32::MIN));
        if let Some(root) = root {
            Self::max_ancestor_diff_rec(root, max.clone());
        } else {
            return 0;
        }
        return *max.borrow();
    }
    fn max_ancestor_diff_rec(root: Rc<RefCell<TreeNode>>, max: Rc<RefCell<i32>>) -> (i32, i32) {
        let root_val = root.borrow().val;
        let left = root
            .borrow()
            .left
            .as_ref()
            .map(|left| Self::max_ancestor_diff_rec(left.clone(), max.clone()));
        let right = root
            .borrow()
            .right
            .as_ref()
            .map(|right| Self::max_ancestor_diff_rec(right.clone(), max.clone()));
        match (left, right) {
            (Some((left_min, left_max)), Some((right_min, right_max))) => {
                let new_max = *[
                    (left_max - root_val).abs(),
                    (left_min - root_val).abs(),
                    (right_min - root_val).abs(),
                    (right_max - root_val).abs(),
                ]
                .iter()
                .max()
                .unwrap_or(&i32::MIN);
                if new_max > *(max.borrow()) {
                    *max.borrow_mut() = new_max;
                    println!(
                        "Updating max to {:?} left_min={:?} left_max={:?} right_min={:?} right_max={:?} root_val={:?}",
                        new_max, left_min, left_max, right_min, right_max, root_val
                    );
                }
                (left_min, right_max)
            }
            (None, Some((right_min, right_max))) => {
                let new_max =
                    std::cmp::max((right_max - root_val).abs(), (right_min - root_val).abs());
                if new_max > *(max.borrow()) {
                    *max.borrow_mut() = new_max;
                    println!(
                        "Updating max to {:?} right_min={:?} right_max={:?} root_val={:?}",
                        new_max, right_min, right_max, root_val
                    );
                }
                (root_val, right_max)
            }
            (Some((left_min, left_max)), None) => {
                let new_max =
                    std::cmp::max((root_val - left_min).abs(), (root_val - left_max).abs());
                if new_max > *(max.borrow()) {
                    *max.borrow_mut() = new_max;
                    println!(
                        "Updating max to {:?} left_min={:?} left_max={:?} root_val={:?}",
                        new_max, left_min, left_max, root_val
                    );
                }
                (left_min, root_val)
            }
            (None, None) => (root_val, root_val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_diff_between_nodes_ancestor() {
        //[8,3,10,1,6,null,14,null,null,4,7,13]
        let node1 = TreeNode::new(1);
        let node4 = TreeNode::new(4);
        let node7 = TreeNode::new(7);
        let node13 = TreeNode::new(13);
        let mut node6 = TreeNode::new(6);
        node6.left = Some(Rc::new(RefCell::new(node4)));
        node6.right = Some(Rc::new(RefCell::new(node7)));
        let mut node3 = TreeNode::new(3);
        node3.left = Some(Rc::new(RefCell::new(node1)));
        node3.right = Some(Rc::new(RefCell::new(node6)));
        let mut node14 = TreeNode::new(14);
        node14.left = Some(Rc::new(RefCell::new(node13)));
        let mut node10 = TreeNode::new(10);
        node10.right = Some(Rc::new(RefCell::new(node14)));
        let mut root = TreeNode::new(8);
        root.left = Some(Rc::new(RefCell::new(node3)));
        root.right = Some(Rc::new(RefCell::new(node10)));
        assert_eq!(
            Solution::max_ancestor_diff(Some(Rc::new(RefCell::new(root)))),
            7
        );
    }

    #[test]
    fn test_max_diff_between_nodes_ancestor2() {
        // [1,null,2,null,0,3]
        let mut node0 = TreeNode::new(0);
        let mut node1 = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        node0.left = Some(Rc::new(RefCell::new(node3)));
        node2.right = Some(Rc::new(RefCell::new(node0)));
        node1.right = Some(Rc::new(RefCell::new(node2)));
        assert_eq!(
            Solution::max_ancestor_diff(Some(Rc::new(RefCell::new(node1)))),
            3
        );
    }
}
