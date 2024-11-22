//simplifies expressions

use crate::{
    trees::{Tree, TreeNode},
    types::symbol::Symbol,
};

// fn simplify<'a>(expr: &mut Tree<Symbol<'a>>) -> Tree<Symbol<'a>> {
//     simplify_add(*expr);
//     simplify_sub(*expr);
//     simplify_mult(*expr);
//     simplify_div(*expr);
// }

// fn simplify_add<'a>(mut expr: Tree<Symbol<'a>>) -> Tree<Symbol<'a>> {
//     match expr.root {
//         Some(mut root_node) => {
//             return Tree {
//                 root: Some(Box::new(simplify_add_rec(*root_node))),
//             };
//         }
//         None => return expr,
//     }
// }
// fn simplify_sub<'a>(mut expr: Tree<Symbol<'a>>) -> Tree<Symbol<'a>> {
//     todo!();
// }
// fn simplify_mult<'a>(mut expr: Tree<Symbol<'a>>) -> Tree<Symbol<'a>> {
//     todo!();
// }
// fn simplify_div<'a>(mut expr: Tree<Symbol<'a>>) -> Tree<Symbol<'a>> {
//     todo!();
// }

// fn simplify_add_rec<'a>(mut expr: TreeNode<Symbol<'a>>) -> TreeNode<Symbol<'a>> {
//     todo!();
// }
// fn simplify_sub_rec<'a>(mut expr: TreeNode<Symbol<'a>>) -> TreeNode<Symbol<'a>> {
//     todo!();
// }
// fn simplify_mult_rec<'a>(mut expr: TreeNode<Symbol<'a>>) -> TreeNode<Symbol<'a>> {
//     todo!();
// }
// fn simplify_div_rec<'a>(mut expr: TreeNode<Symbol<'a>>) -> TreeNode<Symbol<'a>> {
//     todo!();
// }
