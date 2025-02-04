use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use super::trees::{Tree, TreeNode, TreeNodeRef};
use super::vars::{Var, VarTable};

pub(crate) type PostFix<'a> = Result<VecDeque<Symbol>, CASError>;

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

pub(crate) fn into_postfix<'a>(
    tokens: Vec<Token>,
    var_table: &'a VarTable<'a>,
    args: Vec<String>,
) -> PostFix<'a> {
    if tokens.is_empty() {
        //if tokens has length 0
        return Err(CASError {
            kind: CASErrorKind::NoExpressionGiven,
            line_pos: 0,
        });
    }

    let mut output_queue: VecDeque<Symbol> = VecDeque::new();
    let mut operator_stack: VecDeque<Symbol> = VecDeque::new();
    let mut last_token: Option<&TokenType> = None;

    for Token {
        token_type,
        line_pos,
    } in &tokens
    {
        match token_type {
            Name(name) => {
                if let Some(value) = parse_name(
                    &args,
                    name.to_string(),
                    &mut output_queue,
                    var_table,
                    &mut operator_stack,
                    *line_pos,
                ) {
                    return Err(value);
                }
            }
            Num(number) => {
                parse_num(&mut operator_stack, &mut output_queue, &number, &line_pos);

                //if the token is a number put it into the output queue
            }

            Const(name) => {
                output_queue.push_back(Symbol {
                    symbol_type: SymbolType::Const(Const::ResConst(*name)),
                    line_pos: *line_pos,
                });
                if let Some(Symbol {
                    symbol_type: SymbolType::Operator(Neg),
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
                    | Some(Num(_))
                    | Some(Const(_))
                    | Some(Operator(RightBracket))
                    | Some(Operator(RightParen)) => {
                        operator_stack.push_back(Symbol {
                            symbol_type: SymbolType::Operator(Sub),
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
                    Some(Operator(_)) | None => {
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
        }

        last_token = Some(&token_type);
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

    Ok(output_queue)
}

fn parse_num(
    operator_stack: &mut VecDeque<Symbol>,
    output_queue: &mut VecDeque<Symbol>,
    number: &super::CASNum,
    line_pos: &usize,
) {
    if let Some(Symbol {
        symbol_type: SymbolType::Operator(Neg),
        ..
    }) = operator_stack.back()
    {
        operator_stack.pop_back();
        output_queue.push_back(Symbol {
            symbol_type: SymbolType::Num {
                value: -number.clone(),
            },
            line_pos: *line_pos,
        });
    } else {
        output_queue.push_back(Symbol {
            symbol_type: SymbolType::Num {
                value: number.clone(),
            },
            line_pos: *line_pos,
        });
    }
}

pub(crate) fn shunting_yard(output_queue: &mut VecDeque<Symbol>) -> Result<Tree<Symbol>, CASError> {
    let mut tree_stack: Vec<TreeNodeRef<Symbol>> = vec![];
    //temporary stack for constructing the tree

    while let Some(symbol) = output_queue.pop_front() {
        let mut args = vec![];
        for _ in 0..symbol.symbol_type.num_args() {
            if let Some(arg) = tree_stack.pop() {
                args.push(arg);
            } else {
                return Err(CASError {
                    line_pos: symbol.line_pos,
                    kind: CASErrorKind::SyntaxError,
                });
            }
        }
        tree_stack.push(TreeNodeRef::new_from_node(TreeNode {
            data: symbol,
            children: args,
        }));
    }

    if tree_stack.len() > 1 {
        return Err(CASError {
            line_pos: tree_stack[1].0.borrow().data.line_pos,
            kind: CASErrorKind::NoExpressionGiven,
        });
    }

    return match tree_stack.first() {
        None => Err(CASError {
            line_pos: 0,
            kind: CASErrorKind::NoExpressionGiven,
        }),

        //if there are no tokens in tree stack no expression was given
        Some(root_node) => {
            return Ok(Tree {
                root: tree_stack.first().unwrap().clone(),
                //TODO: get rid of this clone
            });
        }
        _ => Err(CASError {
            line_pos: tree_stack[1].0.borrow().data.line_pos,
            kind: CASErrorKind::NoExpressionGiven,
        }),
        //if there are multiple
    };
}

fn parse_right_paren(
    operator_stack: &mut VecDeque<Symbol>,
    output_queue: &mut VecDeque<Symbol>,
    line_pos: usize,
) -> Option<CASError> {
    loop {
        let top_of_stack: Option<&Symbol> = operator_stack.back();
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
        output_queue.push_back(operator_stack.pop_back()?);
    }
    // if there is a function token at the top of the operator stack, then:
    //pop the function from the operator stack into the output queue
    None
}

fn parse_numeric_operator(
    operator_stack: &mut VecDeque<Symbol>,
    o1: &Operator,
    output_queue: &mut VecDeque<Symbol>,
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
                let o2_prec = precedence(o2);
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

fn parse_name(
    args: &[String],
    name: String,
    output_queue: &mut VecDeque<Symbol>,
    var_table: &HashMap<String, Var>,
    operator_stack: &mut VecDeque<Symbol>,
    line_pos: usize,
) -> Option<CASError> {
    //unknown variable name
    let name_symbol = Symbol {
        symbol_type: SymbolType::Variable { name: name.clone() },
        line_pos,
    };
    if args.contains(&name) {
        output_queue.push_back(name_symbol);
        //if the token is a number put it into the output queue
    } else if let Some(var) = var_table.get(&name) {
        match var.args.len() {
            0 => {
                output_queue.push_back(name_symbol);
                if let Some(Symbol {
                    symbol_type: SymbolType::Operator(Neg),
                    ..
                }) = operator_stack.back()
                {
                    output_queue.push_back(operator_stack.pop_back()?);
                }
            }

            //if the token is a number put it into the output queue
            x => operator_stack.push_back(Symbol {
                symbol_type: SymbolType::Function(Func::Function { num_args: x, name }),
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
