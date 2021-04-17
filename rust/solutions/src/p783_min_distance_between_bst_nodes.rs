use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

// TODO:  用stack 怎么做？ref cell 好难用
// struct Solution {}
// impl Solution {
//     pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         let mut nums: Vec<i32> = vec![];
//         let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
//         let mut node = &root;
//         while !stack.is_empty() || node.is_some() {
//             if let Some(r) = node {
//                 stack.push(r.clone());
//                 let v: &TreeNode = &(*r).borrow();
//                 node = &(v.left);
//             } else {
//                 if let Some(head) = stack.pop() {
//                     node = &Some(head.clone());
//                     let v: &TreeNode = &(*node.unwrap()).borrow();
//                     nums.push(v.val);
//                 }
//             }
//         }
//         let mut min = i32::MAX;
//         for i in 0..(nums.len() - 1) {
//             if nums[i+1] - nums[i] < min {
//                 min = nums[i+1] - nums[i];
//             }
//         }
//         min
//     }

// }

struct Solution {}
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nums = vec![];
        Self::dfs(root, &mut nums);
        let mut min = i32::MAX;
        for i in 0..(nums.len() - 1) {
            if nums[i+1] - nums[i] < min {
                min = nums[i+1] - nums[i];
            }
        }
        min
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        if let Some(cell) = root {
            let node = cell.borrow();

            Self::dfs(node.left.clone(), nums);
            nums.push(node.val);
            Self::dfs(node.right.clone(), nums);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_diff_in_bst() {
        let node1 = TreeNode::new(1);
        let node3 = TreeNode::new(3);
        let node2 = TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(node1))),
            right: Some(Rc::new(RefCell::new(node3))),
        };
        let node6 = TreeNode::new(6);
        let node4 = TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(node2))),
            right: Some(Rc::new(RefCell::new(node6))),
        };
        assert_eq!(
            Solution::min_diff_in_bst(Some(Rc::new(RefCell::new(node4)))),
            1
        );
    }
}
