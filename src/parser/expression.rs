use std::collections::VecDeque;

use super::trees::{Parsing, Tree, TreeNode, TreeNodeRef};
use super::vars::{Var, VarTable};
use super::CASNum;

use crate::types::cas_error::CASErrorKind;
use crate::types::symbol::function::Func;
use crate::types::symbol::operator::{
    left_associative, precedence,
    Operator::{self, *},
};
use crate::types::symbol::Symbol;
use crate::types::token::Token::{self, *};
use std::collections::HashMap;

pub fn shunting_yard<'a>(
    tokens: &'a Vec<Token>,
    var_table: VarTable<'a>,
    args: Vec<&str>,
) -> Parsing<'a> {
    let mut output_queue: VecDeque<Symbol> = VecDeque::new();
    let mut operator_stack: VecDeque<Symbol> = VecDeque::new();

    let mut token_iter: std::iter::Peekable<std::slice::Iter<'_, Token>> = tokens.iter().peekable();

    while let Some(token) = token_iter.next() {
        match token {
            Name(name) => {
                if let Some(value) = parse_name(
                    &args,
                    &name,
                    &mut output_queue,
                    &var_table,
                    &mut operator_stack,
                ) {
                    return value;
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

            Operator(o1) => match o1 {
                Add | Sub | Mult | Div | Exp | Less | Greater | Equal | NotEqual | LessEqual
                | GreaterEqual => {
                    if let Some(value) =
                        parse_numeric_operator(&mut operator_stack, &o1, &mut output_queue)
                    {
                        return value;
                    }
                }

                LeftParen | LeftBracket => operator_stack.push_back(Symbol::Operator(*o1)),

                RightParen | RightBracket => {
                    if let Some(value) = parse_right_paren(&mut operator_stack, &mut output_queue) {
                        return value;
                    }
                }
                Comma => {
                    while let Some(Symbol::Operator(o2)) = operator_stack.back() {
                        if *o2 == Operator::LeftParen {
                            break;
                        }
                        //while the operator at the top of the operator stack is not a left parenthesis:

                        output_queue.push_back(Symbol::Operator(*o2));
                        operator_stack.pop_back();
                        //pop the operator from the operator stack into the output queue
                    }
                }
                Assign => {
                    return Err(CASErrorKind::AssignmentInExpression);
                }
            },

            Calc | Sim => todo!(),
            Der => todo!(),
            Integral => todo!(),

            Const(name) => todo!(),
            ResFun(name) => todo!(),
            Error => todo!(),
        }
    }

    /* After the while loop, pop the remaining items from the operator stack into the output queue. */
    // while there are tokens on the operator stack:
    while let Some(token) = operator_stack.pop_back() {
        if token == Symbol::Operator(LeftParen) || token == Symbol::Operator(LeftBracket) {
            return Err(CASErrorKind::MismatchedParentheses);
        }
        output_queue.push_back(token);
    }

    let mut tree_stack: VecDeque<TreeNodeRef<Symbol<'a>>> = VecDeque::new();
    //temporary stack for constructing the tree

    let mut prev_sub: bool = false;
    //if last token was Operator::Sub

    for token in output_queue {
        let mut args = VecDeque::new();
        if Symbol::Operator(Sub) == token {
            //'-' is a special case since it can be both a unary negative operator and a binary subtraction operator
            tree_stack.push_back(Box::new(TreeNode {
                data: token,
                children: VecDeque::new(),
            }));
            prev_sub = true;
            continue;
        }
        if prev_sub {
            let neg = tree_stack.pop_back();
            match neg {
                //check if 0 args so we can unwrap it
                Some(mut operator) => {
                    match tree_stack.len() {
                        1 => {
                            //unary negative
                            operator.children.push_back(Box::new(TreeNode {
                                data: token,
                                children: VecDeque::new(),
                            }));
                            //add next token as child to make -token
                        }
                        2 => {
                            //binary minus

                            let minuend = tree_stack.pop_back().unwrap();
                            //minuend is a fancy word for the value being subtracted from
                            operator.children.push_back(minuend);
                            operator.children.push_back(Box::new(TreeNode {
                                data: token,
                                children: VecDeque::new(),
                            }));
                        }
                        _ => {
                            assert!(false);
                            return Err(CASErrorKind::SyntaxError);
                        } //more than 2 args
                    }

                    tree_stack.push_back(operator);
                }

                None => {
                    assert!(false);
                    return Err(CASErrorKind::SyntaxError);
                } //0 args
            }
        } else {
            match token.num_args() {
                0 => {}
                x => {
                    for _ in 0..x {
                        match tree_stack.pop_back() {
                            Some(symbol) => args.push_front(symbol),
                            //since we're getting them backwards we need to add them backwards
                            None => {
                                {
                                    assert!(false);
                                    return Err(CASErrorKind::SyntaxError);
                                };
                            }
                        }
                    }
                }
            }
            tree_stack.push_back(Box::new(TreeNode {
                data: token,
                children: args,
            }));
        }
    }
    //construct tree
    return match tree_stack.len() {
        0 => Err(CASErrorKind::NoExpressionGiven),

        //if there are no tokens in tree stack no expression was given
        1 => {
            return Ok(Tree {
                root: Some(tree_stack.front().unwrap().clone()),
                //TODO: get rid of this clone
            });
        }
        _ => Err(CASErrorKind::SyntaxError),
        //if there are multiple
    };
}

fn parse_right_paren<'a>(
    operator_stack: &mut VecDeque<Symbol<'a>>,
    output_queue: &mut VecDeque<Symbol<'a>>,
) -> Option<Result<Tree<Symbol<'a>>, CASErrorKind>> {
    loop {
        let top_of_stack: Option<&Symbol<'a>> = operator_stack.back();
        match top_of_stack {
            Some(symbol) => match symbol {
                Symbol::Operator(o2) => match o2 {
                    LeftBracket | LeftParen => break,
                    //while the operator at the top of the operator stack is not a left parenthesis:
                    _ => {
                        output_queue.push_back(Symbol::Operator(*o2));
                        operator_stack.pop_back();
                        //pop the operator from the operator stack into the output queue
                    }
                },
                Symbol::Function(Func::Function { num_args, name }) => {
                    output_queue.push_back(Symbol::Function(Func::Function {
                        num_args: *num_args,
                        name: name.to_string(),
                    }));
                    operator_stack.pop_back();
                    //pop the operator from the operator stack into the output queue
                }
                _ => {
                    return Some(Err(CASErrorKind::SyntaxError));
                }
            },
            None => {
                return Some(Err(CASErrorKind::MismatchedParentheses));
                // assert the operator stack is not empty//
                /* If the stack runs out without finding a left parenthesis, then there are mismatched parentheses. */
            }
        }
    }
    if operator_stack.back() != Some(&Symbol::Operator(LeftParen))
        && operator_stack.back() != Some(&Symbol::Operator(LeftBracket))
    {
        //{assert there is a left parenthesis at the top of the operator stack}
        return Some(Err(CASErrorKind::MismatchedParentheses));
    }
    operator_stack.pop_back();
    //  pop the left parenthesis from the operator stack and discard it
    if let Some(&Symbol::Function { .. }) = operator_stack.back() {
        output_queue.push_back(operator_stack.pop_back().unwrap());
    }
    // if there is a function token at the top of the operator stack, then:
    //pop the function from the operator stack into the output queue
    None
}

fn parse_numeric_operator<'a>(
    operator_stack: &mut VecDeque<Symbol<'a>>,
    o1: &Operator,
    output_queue: &mut VecDeque<Symbol<'a>>,
) -> Option<Parsing<'a>> {
    while let Some(sym) = operator_stack.back() {
        match sym {
            Symbol::Operator(o2) => {
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
            }
            Symbol::Function(Func::Function { num_args, name }) => {
                output_queue.push_back(Symbol::Function(Func::Function {
                    num_args: *num_args,
                    name: name.to_string(),
                }));
                operator_stack.pop_back();
            }
            _ => {
                return Some(Err(CASErrorKind::SyntaxError));
            }
        }

        //pop o2 from the operator stack into the output queue
    }
    operator_stack.push_back(Symbol::Operator(*o1));
    //push o1 onto the operator stack
    None
}

fn parse_name<'a>(
    args: &Vec<&str>,
    name: &'a String,
    output_queue: &mut VecDeque<Symbol<'a>>,
    var_table: &HashMap<&str, Var<'a>>,
    operator_stack: &mut VecDeque<Symbol<'a>>,
) -> Option<Parsing<'a>> {
    //unknown variable name
    if args.contains(&name.as_str()) {
        output_queue.push_back(Symbol::Variable { name });
        //if the token is a number put it into the output queue
    } else if let Some(ref var) = var_table.get(name as &str) {
        match var.args.len() {
            0 => output_queue.push_back(Symbol::Variable { name }),

            //if the token is a number put it into the output queue
            x => operator_stack.push_back(Symbol::Function(Func::Function {
                num_args: x,
                name: name.to_string(),
            })),
            //if the token is a function push it onto the operator stack
        }
    } else {
        return Some(Err(CASErrorKind::UnknownSymbol {
            symbol: name.to_string(),
        }));
    }
    None
}
