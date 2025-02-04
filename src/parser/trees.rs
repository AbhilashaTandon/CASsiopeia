use std::cell::RefCell;
use std::fmt::{Display, Error};
use std::rc::Rc;

use crate::types::cas_error::CASErrorKind;

use crate::types::symbol::Symbol;
use std::hash::{self, Hash, Hasher};
// pub(crate) type TreeNodeRef<T> = Rc<RefCell<TreeNode<T>>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNodeRef<T>(pub Rc<RefCell<TreeNode<T>>>);

impl<T> TreeNodeRef<T> {
    pub fn new(t: T) -> Self {
        TreeNodeRef::new_from_node(TreeNode::from(t))
    }
    pub fn new_from_node(node: TreeNode<T>) -> Self {
        TreeNodeRef(Rc::from(RefCell::from(node)))
    }
}
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

impl Hash for TreeNode<Symbol> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        for child in &self.children {
            child.0.borrow().hash(state);
        }
    }
}
impl Hash for TreeNodeRef<Symbol> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}
//note: I have to be careful with this since hashmaps wont work if the data in here changes
impl Hash for Tree<Symbol> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.root.0.borrow().hash(state);
    }
}

pub(crate) type Parsing<'a> = Result<Tree<Symbol>, CASErrorKind>;

impl<T> From<T> for Tree<T> {
    fn from(value: T) -> Self {
        Tree {
            root: TreeNodeRef::new(value),
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
            root: TreeNodeRef::new_from_node(value),
        }
    }
}

impl<T> TreeNode<T> {
    pub(crate) fn add_child(&mut self, child: TreeNode<T>) {
        self.children.push(TreeNodeRef::new_from_node(child));
    }

    pub(crate) fn add_children(&mut self, children: Vec<TreeNode<T>>) {
        for child in children {
            self.children.push(TreeNodeRef::new_from_node(child));
        }
    }
}

pub(crate) fn construct_node<T>(data: T, children: Vec<T>) -> TreeNode<T> {
    let node_data = data;
    let mut node_children = vec![];
    for child in children {
        node_children.push(TreeNodeRef::new(child));
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
        print_tree_node(f, &self.root.0.borrow(), &mut "".to_string(), true)
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
        print_tree_node(f, &child.0.borrow(), indent, idx == node.children.len() - 1)?
    }
    Ok(())
}
