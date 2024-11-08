use crate::{
    scanner::{TokenItem, Value},
    spec::{left_associative, precedence},
    types::{cas_num::CASNum, error::CASErrorKind},
};
use std::collections::{HashMap, VecDeque};

use crate::spec::TokenType::{
    Add, Assign, Calc, Comma, Const, Der, Div, Eof, Equal, Error, Exp, Float, Greater,
    GreaterEqual, Int, Integral, LeftBracket, LeftParen, Less, LessEqual, Mult, Name, NotEqual,
    ResFun, RightBracket, RightParen, Sim, Sub,
};

pub mod test;

#[derive(PartialEq, Eq, Hash)]
struct TreeNode<T> {
    data: T,
    children: Vec<Box<TreeNode<T>>>,
}

#[derive(PartialEq, Eq, Hash)]
struct Tree<T> {
    //expression
    root: Option<TreeNode<T>>,
}

#[derive(PartialEq)]
struct Var {
    expr: Tree<TokenItem>,
    args: Vec<String>, //if args is empty it is a numeric or symbolic variable, 2, 3, pi, x, etc.
}

//table storing predefined variables (numericals and functions)
type VarTable<'a> = HashMap<String, Var>;

pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Exp,
    LeftBracket,
    LeftParen,
    RightBracket,
    RightParen,
}

pub enum Symbol<'a> {
    //output of parsing
    Variable { name: &'a str },
    Operator(Operator),
    Function { num_args: usize, name: &'a str },
    Num { value: CASNum },
    Const { name: &'a str },
}

pub(crate) struct Parsing<'a> {
    //output of parser
    expr: Tree<Symbol<'a>>,
    error: CASErrorKind,
}

pub(crate) fn shunting_yard<'a>(
    tokens: &'a Vec<TokenItem>,
    var_table: VarTable<'a>,
    args: Vec<String>,
) -> Parsing<'a> {
    let mut output_queue: VecDeque<Symbol> = VecDeque::new();
    let mut operator_stack: VecDeque<Symbol> = VecDeque::new();
    let error = CASErrorKind::NoError;
    let expr: Tree<Symbol> = Tree { root: None };

    let mut token_iter: std::iter::Peekable<std::slice::Iter<'_, TokenItem>> =
        tokens.iter().peekable();

    while let Some(token) = token_iter.next() {
        match token {
            TokenItem::Token {
                token_name,
                token_value,
            } => match token_name {
                Name => todo!(),
                Int => todo!(),
                Float => todo!(),
                Eof => todo!(),
                Assign => todo!(),
                Add => todo!(),
                Sub => todo!(),
                Mult => todo!(),
                Div => todo!(),
                Exp => todo!(),
                LeftParen => todo!(),
                RightParen => todo!(),
                LeftBracket => todo!(),
                RightBracket => todo!(),
                Comma => todo!(),
                Less => todo!(),
                Greater => todo!(),
                Equal => todo!(),
                NotEqual => todo!(),
                LessEqual => todo!(),
                GreaterEqual => todo!(),
                Calc => todo!(),
                Sim => todo!(),
                Der => todo!(),
                Integral => todo!(),
                Const => todo!(),
                ResFun => todo!(),
                Error => todo!(),
            },
            TokenItem::Error { err } => todo!(),
        }
    }

    return Parsing { expr, error };
}

// fn parse_name<'a>(
//     token_value: &'a Option<Value>,
//     var_table: &'a HashMap<String, Var>,
//     output_queue: &'a mut VecDeque<Symbol<'a>>,
//     operator_stack: &'a mut VecDeque<Symbol<'a>>,
//     args: &'a Vec<String>,
//     expr: &'a Tree<Symbol<'a>>,
// ) -> Option<Parsing<'a>> {
//     let var_name: String = (&<Option<Value> as Clone>::clone(&token_value).unwrap()).to_string();
//     if let Some(var) = var_table.get(&var_name) {
//         match var.args.len() {
//             0 => output_queue.push_back(Symbol::Variable { name: &var_name }), //variable
//             x => operator_stack.push_back(Symbol::Function {
//                 num_args: x,
//                 name: &var_name,
//             }), //function
//         }
//     } else if args.contains(&var_name) {
//         output_queue.push_back(Symbol::Variable { name: &var_name })
//     } else {
//         return Some(Parsing {
//             expr: *expr,
//             error: CASErrorKind::UnknownSymbol,
//         });
//     }
//     None
// }
