use std::collections::VecDeque;

use crate::types::error::CASErrorKind;

use crate::spec::Symbol;

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
