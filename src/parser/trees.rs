use std::collections::VecDeque;

use crate::types::cas_error::CASErrorKind;

use crate::types::symbol::Symbol;

pub type TreeNodeRef<T> = Box<TreeNode<T>>;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct TreeNode<T> {
    pub data: T,
    pub children: VecDeque<TreeNodeRef<T>>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Tree<T> {
    //expression
    pub root: Option<TreeNodeRef<T>>,
    //option allows us to have empty trees
}

pub type Parsing<'a> = Result<Tree<Symbol<'a>>, CASErrorKind>;

impl<T> From<T> for Tree<T> {
    fn from(value: T) -> Self {
        return Tree {
            root: Some(Box::new(TreeNode::from(value))),
        };
    }
}

impl<T> From<T> for TreeNode<T> {
    fn from(value: T) -> Self {
        TreeNode {
            data: value,
            children: VecDeque::new(),
        }
    }
}

impl<T> From<TreeNode<T>> for Tree<T> {
    fn from(value: TreeNode<T>) -> Self {
        return Tree {
            root: Some(Box::new(value)),
        };
    }
}

impl<T> TreeNode<T> {
    pub fn add_child(&mut self, child: T) {
        self.children.push_back(Box::new(TreeNode::from(child)));
    }

    pub fn add_children(&mut self, children: Vec<T>) {
        for child in children {
            self.children.push_back(Box::new(TreeNode::from(child)));
        }
    }
}

pub fn construct_tree<T>(data: T, children: Vec<T>) -> Tree<T> {
    let mut root = TreeNode::from(data);
    root.add_children(children);
    return Tree::from(root);
}
