use crate::spec::{precedence, Operator::*};
use crate::{
    scanner::TokenItem,
    spec::{left_associative, Operator},
    types::{cas_num::CASNum, error::CASErrorKind},
};
use std::collections::{HashMap, VecDeque};

use crate::spec::TokenType::*;

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
struct Var<'a> {
    expr: Tree<TokenItem>,
    args: Vec<&'a str>, //if args is empty it is a numeric or symbolic variable, 2, 3, pi, x, etc.
}

//table storing predefined variables (numericals and functions)
type VarTable<'a> = HashMap<&'a str, Var<'a>>;

#[derive(Debug, PartialEq)]
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
    args: Vec<&str>,
) -> Parsing<'a> {
    let mut output_queue: VecDeque<Symbol> = VecDeque::new();
    let mut operator_stack: VecDeque<Symbol> = VecDeque::new();
    let error = CASErrorKind::NoError;
    let expr: Tree<Symbol> = Tree { root: None };

    let mut token_iter: std::iter::Peekable<std::slice::Iter<'_, TokenItem>> =
        tokens.iter().peekable();

    while let Some(token) = token_iter.next() {
        match token {
            TokenItem::Token(token_type) => match token_type {
                Name(name) => {
                    if args.contains(&name.as_str()) {
                        output_queue.push_back(Symbol::Variable { name });
                        //if the token is a number put it into the output queue
                    } else if let Some(ref var) = var_table.get(name as &str) {
                        match var.args.len() {
                            0 => output_queue.push_back(Symbol::Variable { name }),

                            //if the token is a number put it into the output queue
                            x => operator_stack.push_back(Symbol::Function { num_args: x, name }),
                            //if the token is a function push it onto the operator stack
                        }
                    } else {
                        return Parsing {
                            expr,
                            error: CASErrorKind::UnknownSymbol,
                        };
                        //unknown variable name
                    }
                }
                Int(i) => {
                    output_queue.push_back(Symbol::Num {
                        value: CASNum::from(*i),
                    });
                    //if the token is a number put it into the output queue
                }
                Float(f) => {
                    output_queue.push_back(Symbol::Num {
                        value: CASNum::from(*f),
                    });
                    //if the token is a number put it into the output queue
                }
                Eof => {
                    break;
                }
                Assign => {
                    return Parsing {
                        expr,
                        error: CASErrorKind::AssignmentInExpression,
                    };
                }
                Operator(o1) => match o1 {
                    Add | Sub | Mult | Div | Exp | Less | Greater | Equal | NotEqual
                    | LessEqual | GreaterEqual => {
                        while let Some(Symbol::Operator(o2)) = operator_stack.back() {
                            if *o2 == Operator::LeftParen {
                                break;
                            }
                            //while there is an operator at the top of the stack which is not a left paren
                            //or  (o2 has <= precedence than o1 and (o1 and o2 have diff precedence or o1 is not left-associative))
                            //

                            let o1_prec = precedence(o1);
                            let o2_prec = precedence(o2);
                            if o2_prec <= o1_prec && (o2_prec != o1_prec || !left_associative(o1)) {
                                break;
                            }
                            output_queue.push_back(Symbol::Operator(*o2));
                            operator_stack.pop_back();
                            //pop o2 from the operator stack into the output queue
                        }
                        operator_stack.push_back(Symbol::Operator(*o1));
                        //push o1 onto the operator stack
                    }

                    LeftParen | LeftBracket => todo!(),

                    RightParen | RightBracket => todo!(),
                },

                Comma => todo!(),

                Calc => todo!(),
                Sim => todo!(),
                Der => todo!(),
                Integral => todo!(),

                Const(name) => todo!(),
                ResFun(name) => todo!(),
                Error => todo!(),
            },
            TokenItem::Error(err) => todo!(),
        }
    }

    return Parsing { expr, error };
}
