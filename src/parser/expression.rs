use std::collections::VecDeque;

use num_traits::Float;

use super::trees::{Parsing, Tree, TreeNode, TreeNodeRef};
use super::vars::{Var, VarTable};
use super::CASNum;

pub type PostFix<'a> = Result<VecDeque<Symbol<'a>>, CASError>;

use crate::types::cas_error::{CASError, CASErrorKind};
use crate::types::symbol::constant::Const;
use crate::types::symbol::function::Func;
use crate::types::symbol::operator::{
    left_associative, precedence,
    Operator::{self, *},
};
use crate::types::symbol::{Symbol, SymbolType};
use crate::types::token::Token;
use crate::types::token::TokenType::{self, *};
use std::collections::HashMap;

pub fn to_postfix<'a>(
    tokens: &'a Vec<Token>,
    var_table: &'a VarTable<'a>,
    args: Vec<&str>,
) -> PostFix<'a> {
    let mut token_iter: std::iter::Peekable<std::slice::Iter<'_, Token>> = tokens.iter().peekable();

    if token_iter.peek().is_none() {
        //if tokens has length 0
        return Err(CASError {
            kind: CASErrorKind::NoExpressionGiven,
            line_pos: 0,
        });
    }

    let mut output_queue: VecDeque<Symbol> = VecDeque::new();
    let mut operator_stack: VecDeque<Symbol> = VecDeque::new();
    let mut last_token: Option<TokenType> = None;

    while let Some(Token {
        token_type,
        line_pos,
    }) = token_iter.next()
    {
        match token_type {
            Name(name) => {
                if let Some(value) = parse_name(
                    &args,
                    &name,
                    &mut output_queue,
                    &var_table,
                    &mut operator_stack,
                    *line_pos,
                ) {
                    return Err(value);
                }
            }
            Int(i) => {
                if let Some(Symbol {
                    symbol_type: SymbolType::Operator(Sub),
                    ..
                }) = operator_stack.back()
                {
                    operator_stack.pop_back();
                    output_queue.push_back(Symbol {
                        symbol_type: SymbolType::Num {
                            value: CASNum::from(-*i),
                        },
                        line_pos: *line_pos,
                    });
                } else {
                    output_queue.push_back(Symbol {
                        symbol_type: SymbolType::Num {
                            value: CASNum::from(-*i),
                        },
                        line_pos: *line_pos,
                    });
                }

                //if the token is a number put it into the output queue
            }
            Float(f) => {
                if let Some(Symbol {
                    symbol_type: SymbolType::Operator(Sub),
                    ..
                }) = operator_stack.back()
                {
                    operator_stack.pop_back();
                    output_queue.push_back(Symbol {
                        symbol_type: SymbolType::Num {
                            value: CASNum::from(-*f),
                        },
                        line_pos: *line_pos,
                    });
                } else {
                    output_queue.push_back(Symbol {
                        symbol_type: SymbolType::Num {
                            value: CASNum::from(-*f),
                        },
                        line_pos: *line_pos,
                    });
                }

                //if the token is a number put it into the output queue
            }

            Const(name) => {
                output_queue.push_back(Symbol {
                    symbol_type: SymbolType::Const(Const::ResConst(*name)),
                    line_pos: *line_pos,
                });
                if let Some(Symbol {
                    symbol_type: SymbolType::Operator(Sub),
                    ..
                }) = operator_stack.back()
                {
                    output_queue.push_back(operator_stack.pop_back().unwrap());
                }
            }
            ResFun(name) => operator_stack.push_back(Symbol {
                symbol_type: SymbolType::Function(Func::ResFun(*name)),
                line_pos: *line_pos,
            }),

            Calc | Sim => {
                return Err(CASError {
                    kind: CASErrorKind::CommandInExpression {
                        command: Token {
                            token_type: token_type.clone(),
                            line_pos: *line_pos,
                        },
                    },
                    line_pos: *line_pos,
                });
            }

            Operator(o1) => match o1 {
                Add | Mult | Div | Exp | Less | Greater | Equal | NotEqual | LessEqual
                | GreaterEqual => {
                    if let Some(value) = parse_numeric_operator(
                        &mut operator_stack,
                        &o1,
                        &mut output_queue,
                        *line_pos,
                    ) {
                        return Err(value);
                    }
                }

                LeftParen | LeftBracket => operator_stack.push_back(Symbol {
                    symbol_type: SymbolType::Operator(*o1),
                    line_pos: *line_pos,
                }),

                RightParen | RightBracket => {
                    if let Some(value) =
                        parse_right_paren(&mut operator_stack, &mut output_queue, *line_pos)
                    {
                        return Err(value);
                    }
                }
                Comma => {
                    while let Some(o2) = operator_stack.pop_back() {
                        if o2.symbol_type == SymbolType::Operator(Operator::LeftParen) {
                            operator_stack.push_back(o2);
                            break;
                        }
                        //while the operator at the top of the operator stack is not a left parenthesis:

                        output_queue.push_back(o2);
                        //pop the operator from the operator stack into the output queue
                    }
                }
                Assign => {
                    return Err(CASError {
                        kind: CASErrorKind::AssignmentInExpression,
                        line_pos: *line_pos,
                    });
                }
                Sub | Neg => match last_token {
                    Some(Name(_))
                    | Some(Int(_))
                    | Some(Float(_))
                    | Some(Const(_))
                    | Some(Operator(RightBracket))
                    | Some(Operator(RightParen)) => {
                        operator_stack.push_back(Symbol {
                            symbol_type: SymbolType::Operator(Sub),
                            line_pos: *line_pos,
                        });
                    }
                    Some(Calc) | Some(Sim) => {
                        return Err(CASError {
                            kind: CASErrorKind::CommandInExpression {
                                command: Token {
                                    token_type: token_type.clone(),
                                    line_pos: *line_pos,
                                },
                            },
                            line_pos: *line_pos,
                        });
                    }
                    Some(Eof) => {
                        return Err(CASError {
                            kind: CASErrorKind::SyntaxError,
                            line_pos: *line_pos,
                        });
                    }
                    Some(ResFun(f)) => {
                        return Err(CASError {
                            kind: CASErrorKind::WrongNumberOfArgs {
                                args_given: 0,
                                args_needed: f.num_args(),
                                func_name: f.to_string(),
                            },
                            line_pos: *line_pos,
                        })
                    }
                    Some(Der) | Some(Integral) | Some(Operator(_)) | None => {
                        operator_stack.push_back(Symbol {
                            symbol_type: SymbolType::Operator(Neg),
                            line_pos: *line_pos,
                        });
                    }
                },
            },

            Eof => {
                break;
            }
            Der => todo!(),
            Integral => todo!(),
        }

        last_token = Some(token_type.clone());

        // print!("output queue: [");
        // for token in &output_queue {
        //     print!("{} ", token.to_string());
        // }
        // println!("]");
        // print!("operator stack: [");
        // for token in &operator_stack {
        //     print!("{} ", token.to_string());
        // }
        // println!("]");
        // println!();
    }

    /* After the while loop, pop the remaining items from the operator stack into the output queue. */
    // while there are tokens on the operator stack:
    while let Some(token) = operator_stack.pop_back() {
        if token.symbol_type == SymbolType::Operator(LeftParen)
            || token.symbol_type == SymbolType::Operator(LeftBracket)
        {
            return Err(CASError {
                line_pos: 0,
                kind: CASErrorKind::MismatchedParentheses,
            });
        }
        output_queue.push_back(token);
    }

    return Ok(output_queue);
}

// pub fn shunting_yard() {
//     let mut tree_stack: VecDeque<TreeNodeRef<Symbol<'a>>> = VecDeque::new();
//     //temporary stack for constructing the tree

//     let mut prev_sub: bool = false;
//     //if last token was Operator::Sub

//     for token in &output_queue {
//         print!("{}, ", token);
//     }
//     println!();

//     for token in output_queue {
//         for token_2 in &tree_stack {
//             print!("{}, ", token_2.data);
//         }
//         println!();
//         let mut args = VecDeque::new();
//         if SymbolType::Operator(Sub) == token {
//             //'-' is a special case since it can be both a unary negative operator and a binary subtraction operator
//             tree_stack.push_back(Box::new(TreeNode {
//                 data: token,
//                 children: VecDeque::new(),
//             }));
//             prev_sub = true;
//             continue;
//         } else if prev_sub {
//             let neg = tree_stack.pop_back();
//             match neg {
//                 //check if 0 args so we can unwrap it
//                 Some(mut operator) => {
//                     match tree_stack.len() {
//                         1 => {
//                             //unary negative
//                             operator.children.push_back(Box::new(TreeNode {
//                                 data: token,
//                                 children: VecDeque::new(),
//                             }));
//                             //add next token as child to make -token
//                         }
//                         2 => {
//                             //binary minus

//                             let minuend = tree_stack.pop_back().unwrap();
//                             //minuend is a fancy word for the value being subtracted from
//                             operator.children.push_back(minuend);
//                             operator.children.push_back(Box::new(TreeNode {
//                                 data: token,
//                                 children: VecDeque::new(),
//                             }));
//                         }
//                         _ => {
//                             assert!(false);
//                             return Err(CASErrorKind::SyntaxError);
//                         } //more than 2 args
//                     }

//                     tree_stack.push_back(operator);
//                 }

//                 None => {
//                     assert!(false);
//                     return Err(CASErrorKind::SyntaxError);
//                 } //0 args
//             }
//         } else {
//             assert_ne!(token, SymbolType::Operator(Operator::Sub));
//             match token.num_args() {
//                 0 => {}
//                 x => {
//                     for _ in 0..x {
//                         match tree_stack.pop_back() {
//                             Some(symbol) => args.push_front(symbol),
//                             //since we're getting them backwards we need to add them backwards
//                             None => {
//                                 println!("{}", token);
//                                 assert!(false);
//                                 return Err(CASErrorKind::SyntaxError);
//                             }
//                         }
//                     }
//                 }
//             }
//             tree_stack.push_back(Box::new(TreeNode {
//                 data: token,
//                 children: args,
//             }));
//         }
//     }
//     //construct tree
//     return match tree_stack.len() {
//         0 => Err(CASErrorKind::NoExpressionGiven),

//         //if there are no tokens in tree stack no expression was given
//         1 => {
//             return Ok(Tree {
//                 root: Some(tree_stack.front().unwrap().clone()),
//                 //TODO: get rid of this clone
//             });
//         }
//         _ => Err(CASErrorKind::SyntaxError),
//         //if there are multiple
//     };
// }

fn parse_right_paren<'a>(
    operator_stack: &mut VecDeque<Symbol<'a>>,
    output_queue: &mut VecDeque<Symbol<'a>>,
    line_pos: usize,
) -> Option<CASError> {
    loop {
        let top_of_stack: Option<&Symbol<'a>> = operator_stack.back();
        match top_of_stack {
            Some(symbol) => match &symbol.symbol_type {
                SymbolType::Operator(o2) => match o2 {
                    LeftBracket | LeftParen => break,
                    //while the operator at the top of the operator stack is not a left parenthesis:
                    _ => {
                        output_queue.push_back(symbol.clone());
                        operator_stack.pop_back();
                        //pop the operator from the operator stack into the output queue
                    }
                },
                SymbolType::Function(Func::Function { .. }) => {
                    output_queue.push_back(symbol.clone());
                    operator_stack.pop_back();
                    //pop the operator from the operator stack into the output queue
                }
                _ => {
                    return Some(CASError {
                        line_pos,
                        kind: CASErrorKind::SyntaxError,
                    });
                }
            },
            None => {
                return Some(CASError {
                    line_pos,
                    kind: CASErrorKind::MismatchedParentheses,
                });
                // assert the operator stack is not empty//
                /* If the stack runs out without finding a left parenthesis, then there are mismatched parentheses. */
            }
        }
    }
    match operator_stack.back() {
        Some(symbol) => match symbol.symbol_type {
            SymbolType::Operator(LeftParen) | SymbolType::Operator(LeftBracket) => {
                //{assert there is a left parenthesis at the top of the operator stack}
            }
            _ => {
                return Some(CASError {
                    line_pos,
                    kind: CASErrorKind::MismatchedParentheses,
                });
            }
        },
        None => {
            return Some(CASError {
                line_pos,
                kind: CASErrorKind::MismatchedParentheses,
            });
            //{assert there is a left parenthesis at the top of the operator stack}
        }
    }

    operator_stack.pop_back();
    //  pop the left parenthesis from the operator stack and discard it
    if let Some(Symbol {
        symbol_type: SymbolType::Function { .. },
        ..
    }) = operator_stack.back()
    {
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
    line_pos: usize,
) -> Option<CASError> {
    while let Some(sym) = operator_stack.back() {
        match &sym.symbol_type {
            SymbolType::Operator(o2) => {
                if *o2 == Operator::LeftParen {
                    break;
                }
                //while there is an operator at the top of the stack which is not a left paren
                //or  (o2 has <= precedence than o1 and (o1 and o2 have diff precedence or o1 is not left-associative))
                //

                let o1_prec = precedence(o1);
                let o2_prec = precedence(&o2);
                if o2_prec <= o1_prec && (o2_prec != o1_prec || !left_associative(o1)) {
                    break;
                }
                output_queue.push_back(sym.clone());
                operator_stack.pop_back();
            }
            SymbolType::Function(Func::Function { .. }) => {
                output_queue.push_back(sym.clone());
                operator_stack.pop_back();
            }
            _ => {
                return Some(CASError {
                    kind: CASErrorKind::SyntaxError,
                    line_pos,
                });
            }
        }

        //pop o2 from the operator stack into the output queue
    }
    operator_stack.push_back(Symbol {
        symbol_type: SymbolType::Operator(*o1),
        line_pos,
    });
    //push o1 onto the operator stack
    None
}

fn parse_name<'a>(
    args: &Vec<&str>,
    name: &'a String,
    output_queue: &mut VecDeque<Symbol<'a>>,
    var_table: &HashMap<&str, Var<'a>>,
    operator_stack: &mut VecDeque<Symbol<'a>>,
    line_pos: usize,
) -> Option<CASError> {
    //unknown variable name
    let name_symbol = Symbol {
        symbol_type: SymbolType::Variable { name },
        line_pos,
    };
    if args.contains(&name.as_str()) {
        output_queue.push_back(name_symbol);
        //if the token is a number put it into the output queue
    } else if let Some(ref var) = var_table.get(name as &str) {
        match var.args.len() {
            0 => {
                output_queue.push_back(name_symbol);
                if let Some(Symbol {
                    symbol_type: SymbolType::Operator(Sub),
                    ..
                }) = operator_stack.back()
                {
                    output_queue.push_back(operator_stack.pop_back().unwrap());
                }
            }

            //if the token is a number put it into the output queue
            x => operator_stack.push_back(Symbol {
                symbol_type: SymbolType::Function(Func::Function {
                    num_args: x,
                    name: name.to_string(),
                }),
                line_pos,
            }),
            //if the token is a function push it onto the operator stack
        }
    } else {
        let kind = CASErrorKind::UnknownSymbol {
            symbol: name.to_string(),
        };
        return Some(CASError { line_pos, kind });
    }
    None
}
