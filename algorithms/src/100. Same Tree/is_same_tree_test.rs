use super::*;

use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn is_same_tree_case1() {
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    let expected = true;
    assert_eq!(Solution::is_same_tree(tree1, tree2), expected);
}

#[test]
fn is_same_tree_case2() {
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: None,
    })));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    let expected = false;
    assert_eq!(Solution::is_same_tree(tree1, tree2), expected);
}

#[test]
fn is_same_tree_case3() {
    let tree1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    })));
    let tree2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    })));
    let expected = false;
    assert_eq!(Solution::is_same_tree(tree1, tree2), expected);
}
