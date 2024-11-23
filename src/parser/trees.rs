use std::cell::RefCell;
use std::fmt::{Display, Error};
use std::rc::Rc;

use crate::types::cas_error::CASErrorKind;

use crate::types::symbol::Symbol;

pub(crate) type TreeNodeRef<T> = Rc<RefCell<TreeNode<T>>>;

#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) struct TreeNode<T> {
    pub(crate) data: T,
    pub(crate) children: Vec<TreeNodeRef<T>>,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct Tree<T> {
    //expression
    pub(crate) root: TreeNodeRef<T>,
}

pub(crate) type Parsing<'a> = Result<Tree<Symbol>, CASErrorKind>;

impl<T> From<T> for Tree<T> {
    fn from(value: T) -> Self {
        Tree {
            root: Rc::from(RefCell::from(TreeNode::from(value))),
        }
    }
}

impl<T> From<T> for TreeNode<T> {
    fn from(value: T) -> Self {
        TreeNode {
            data: value,
            children: vec![],
        }
    }
}

impl<T> From<TreeNode<T>> for Tree<T> {
    fn from(value: TreeNode<T>) -> Self {
        Tree {
            root: Rc::from(RefCell::from(value)),
        }
    }
}

impl<T> TreeNode<T> {
    pub(crate) fn add_child(&mut self, child: TreeNode<T>) {
        self.children.push(Rc::from(RefCell::from(child)));
    }

    pub(crate) fn add_children(&mut self, children: Vec<TreeNode<T>>) {
        for child in children {
            self.children.push(Rc::from(RefCell::from(child)));
        }
    }
}

pub(crate) fn construct_node<T>(data: T, children: Vec<T>) -> TreeNode<T> {
    let node_data = data;
    let mut node_children = vec![];
    for child in children {
        node_children.push(Rc::from(RefCell::from(TreeNode::from(child))));
    }
    TreeNode {
        data: node_data,
        children: node_children,
    }
}

pub(crate) fn construct_tree<T>(data: T, children: Vec<TreeNode<T>>) -> Tree<T> {
    let mut root = TreeNode::from(data);
    root.add_children(children);
    Tree::from(root)
}

impl<Symbol: std::fmt::Display> Display for Tree<Symbol> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print_tree_node(f, &self.root.borrow(), &mut "".to_string(), true)
    }
}

//adapted from https://stackoverflow.com/questions/36311991/c-sharp-display-a-binary-search-tree-in-console/36313190#36313190
fn print_tree_node<Symbol: std::fmt::Display>(
    f: &mut std::fmt::Formatter<'_>,
    node: &TreeNode<Symbol>,
    indent: &mut String,
    last: bool,
) -> std::fmt::Result {
    write!(f, "{}", indent)?;
    if last {
        write!(f, "└─")?;
        *indent += " ";
    } else {
        write!(f, "├─")?;
        *indent += "| ";
    }
    writeln!(f, " {}", node.data)?;

    for (idx, child) in node.children.iter().enumerate() {
        print_tree_node(f, &child.borrow(), indent, idx == node.children.len() - 1)?
    }
    Ok(())
}
