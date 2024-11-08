use crate::spec::{precedence, Operator::*};
use crate::{
    scanner::TokenItem,
    spec::{left_associative, Operator},
    types::{cas_num::CASNum, error::CASErrorKind},
};
use std::collections::{HashMap, VecDeque};

use crate::spec::TokenType::*;

mod test;

type TreeNodeRef<T> = Box<TreeNode<T>>;

#[derive(PartialEq, Eq, Hash, Debug)]
struct TreeNode<T> {
    data: T,
    children: VecDeque<TreeNodeRef<T>>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Tree<T> {
    //expression
    root: Option<TreeNodeRef<T>>,
}

#[derive(PartialEq, Debug)]
struct Var<'a> {
    expr: Tree<TokenItem>,
    args: VecDeque<&'a str>, //if args is empty it is a numeric or symbolic variable, 2, 3, pi, x, etc.
}

//table storing predefined variables (numericals and functions)
type VarTable<'a> = HashMap<&'a str, Var<'a>>;
type Parsing<'a> = Result<Tree<Symbol<'a>>, CASErrorKind>;

#[derive(Debug, PartialEq)]
enum Symbol<'a> {
    //output of parsing
    Variable { name: &'a str },
    Operator(Operator),
    Function { num_args: usize, name: &'a str },
    Num { value: CASNum },
    Const { name: &'a str },
}

impl Symbol<'_> {
    fn num_args(&self) -> usize {
        match self {
            Symbol::Variable { .. } => 0,
            Symbol::Operator(..) => 2,
            Symbol::Function { num_args, .. } => *num_args,
            Symbol::Num { .. } => 0,
            Symbol::Const { .. } => 0,
        }
    }
}

fn shunting_yard<'a>(
    tokens: &'a Vec<TokenItem>,
    var_table: VarTable<'a>,
    args: Vec<&str>,
) -> Parsing<'a> {
    let mut output_queue: VecDeque<Symbol> = VecDeque::new();
    let mut operator_stack: VecDeque<Symbol> = VecDeque::new();

    let mut token_iter: std::iter::Peekable<std::slice::Iter<'_, TokenItem>> =
        tokens.iter().peekable();

    while let Some(token) = token_iter.next() {
        match token {
            TokenItem::Token(token_type) => match token_type {
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
                Assign => {
                    return Err(CASErrorKind::AssignmentInExpression);
                }
                Operator(o1) => match o1 {
                    Add | Sub | Mult | Div | Exp | Less | Greater | Equal | NotEqual
                    | LessEqual | GreaterEqual => {
                        if let Some(value) =
                            parse_numeric_operator(&mut operator_stack, &o1, &mut output_queue)
                        {
                            return value;
                        }
                    }

                    LeftParen | LeftBracket => operator_stack.push_back(Symbol::Operator(*o1)),

                    RightParen | RightBracket => {
                        loop {
                            let top_of_stack: Option<&Symbol<'_>> = operator_stack.back();
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
                                    Symbol::Function { num_args, name } => {
                                        output_queue.push_back(Symbol::Function {
                                            num_args: *num_args,
                                            name,
                                        });
                                        operator_stack.pop_back();
                                        //pop the operator from the operator stack into the output queue
                                    }
                                    _ => {
                                        return Err(CASErrorKind::SyntaxError);
                                    }
                                },
                                None => {
                                    return Err(CASErrorKind::MismatchedParentheses);
                                    // assert the operator stack is not empty//
                                    /* If the stack runs out without finding a left parenthesis, then there are mismatched parentheses. */
                                }
                            }
                        }
                        if operator_stack.back() != Some(&Symbol::Operator(LeftParen))
                            && operator_stack.back() != Some(&Symbol::Operator(LeftBracket))
                        {
                            //{assert there is a left parenthesis at the top of the operator stack}
                            return Err(CASErrorKind::MismatchedParentheses);
                        }
                        operator_stack.pop_back();
                        //  pop the left parenthesis from the operator stack and discard it
                        if let Some(&Symbol::Function { .. }) = operator_stack.back() {
                            output_queue.push_back(operator_stack.pop_back().unwrap());
                        }

                        // if there is a function token at the top of the operator stack, then:
                        //pop the function from the operator stack into the output queue
                    }
                },

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

                Calc | Sim => todo!(),
                Der => todo!(),
                Integral => todo!(),

                Const(name) => todo!(),
                ResFun(name) => todo!(),
                Error => todo!(),
            },
            TokenItem::Error(err) => todo!(),
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

    for token in output_queue {
        match token.num_args() {
            0 => tree_stack.push_back(Box::new(TreeNode {
                data: token,
                children: VecDeque::new(),
            })),
            x => {
                let mut args = VecDeque::new();
                for _ in 0..x {
                    match tree_stack.pop_back() {
                        Some(symbol) => args.push_front(symbol),
                        //since we're getting them backwards we need to add them backwards
                        None => {
                            return Err(CASErrorKind::SyntaxError);
                        }
                    }
                }
                tree_stack.push_back(Box::new(TreeNode {
                    data: token,
                    children: args,
                }));
            }
        }
    }
    //construct tree
    return match tree_stack.front() {
        Some(expr) => Ok(Tree { root: Some(*expr) }),
        None => Err(CASErrorKind::NoExpressionGiven),
    };
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
            Symbol::Function { num_args, name } => {
                output_queue.push_back(Symbol::Function {
                    num_args: *num_args,
                    name,
                });
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
            x => operator_stack.push_back(Symbol::Function { num_args: x, name }),
            //if the token is a function push it onto the operator stack
        }
    } else {
        return Some(Err(CASErrorKind::UnknownSymbol));
    }
    None
}
