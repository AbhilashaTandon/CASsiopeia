use std::cell::RefCell;
use std::fmt::{Display, Error};
use std::rc::Rc;

use crate::types::cas_error::CASErrorKind;
use crate::types::symbol::Symbol;

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
pub(crate) struct ASTNode {
    pub(crate) data: Symbol,
    pub(crate) children: Vec<usize>,
}
// Nodes are referenced based on their indices in this list

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
pub(crate) struct AST(pub(crate) Vec<ASTNode>);

impl AST {
    pub(crate) fn new(root: Symbol) -> Self {
        let mut new_ast = AST(vec![
            (ASTNode {
                data: root,
                children: vec![],
            }),
        ]);
        new_ast
    }

    pub(crate) fn add_node(self: &mut Self, value: Symbol, parent: usize) -> Option<usize> {
        let length = self.0.len();
        if (parent > length) {
            None
        } else {
            self.0.push(ASTNode {
                data: value,
                children: vec![],
            });
            self.0[parent].children.push(length);
            Some(length)
        }
    }

    pub(crate) fn get_node(self: &Self, index: usize) -> Option<&ASTNode> {
        if (index > self.0.len()) {
            None
        } else {
            Some(&self.0[index])
        }
    }

    pub(crate) fn get_root(self: &Self) -> Option<&ASTNode> {
        if (self.0.len() > 0) {
            Some(&self.0[0])
        } else {
            None
        }
    }

    pub(crate) fn get_data(self: &Self, index: usize) -> Option<&Symbol> {
        if (index > self.0.len()) {
            None
        } else {
            Some(&self.0[index].data)
        }
    }

    pub(crate) fn set_data(self: &mut Self, index: usize, data: Symbol) -> bool {
        if (index > self.0.len()) {
            false
        } else {
            self.0[index].data = data;
            true
        }
    }

    pub(crate) fn get_length(self: &Self) -> usize {
        return self.0.len();
    }

    pub(crate) fn get_num_children(self: &Self, index: usize) -> usize {
        return self.0[index].children.len();
    }

    pub(crate) fn get_children(self: &Self, index: usize) -> &Vec<usize> {
        return &self.0[index].children;
    }
}
