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
    pub fn add_child(&mut self, child: TreeNode<T>) {
        self.children.push_back(Box::new(child));
    }

    pub fn add_children(&mut self, children: Vec<TreeNode<T>>) {
        for child in children {
            self.children.push_back(Box::new(child));
        }
    }
}

pub fn construct_node<T>(data: T, children: Vec<T>) -> TreeNode<T> {
    let node_data = data;
    let mut node_children = VecDeque::new();
    for child in children {
        node_children.push_back(Box::new(TreeNode::from(child)));
    }
    return TreeNode {
        data: node_data,
        children: node_children,
    };
}

pub fn construct_tree<T>(data: T, children: Vec<TreeNode<T>>) -> Tree<T> {
    let mut root = TreeNode::from(data);
    root.add_children(children);
    return Tree::from(root);
}
