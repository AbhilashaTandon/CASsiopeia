use crate::{
    scanner::{TokenItem, Value},
    types::error::CASErrorKind,
};
use std::collections::HashMap;

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

#[derive(PartialEq, Hash)]
struct Var {
    expr: Tree<TokenItem>,
    args: Vec<String>, //if args is empty it is a numeric or symbolic variable, 2, 3, pi, x, etc.
}

//table storing predefined variables (numericals and functions)
type VarTable<'a> = HashMap<String, Var>;

/*
/* The functions referred to in this algorithm are simple single argument functions such as sine, inverse or factorial. */
/* This implementation does not implement composite functions, functions with a variable number of arguments, or unary operators. */


while there are tokens to be read:
    read a token
    if the token is:
    - a number:
        put it into the output queue
    - a function:
        push it onto the operator stack
    - an operator o1:
        while (
            there is an operator o2 at the top of the operator stack which is not a left parenthesis,
            and (o2 has greater precedence than o1 or (o1 and o2 have the same precedence and o1 is left-associative))
        ):
            pop o2 from the operator stack into the output queue
        push o1 onto the operator stack
    - a ",":
        while the operator at the top of the operator stack is not a left parenthesis:
             pop the operator from the operator stack into the output queue
    - a left parenthesis (i.e. "("):
        push it onto the operator stack
    - a right parenthesis (i.e. ")"):
        while the operator at the top of the operator stack is not a left parenthesis:
            {assert the operator stack is not empty}
            /* If the stack runs out without finding a left parenthesis, then there are mismatched parentheses. */
            pop the operator from the operator stack into the output queue
        {assert there is a left parenthesis at the top of the operator stack}
        pop the left parenthesis from the operator stack and discard it
        if there is a function token at the top of the operator stack, then:
            pop the function from the operator stack into the output queue
/* After the while loop, pop the remaining items from the operator stack into the output queue. */
while there are tokens on the operator stack:
    /* If the operator token on the top of the stack is a parenthesis, then there are mismatched parentheses. */
    {assert the operator on top of the stack is not a (left) parenthesis}
    pop the operator from the operator stack onto the output queue
*/

pub(crate) struct Parsing {
    //output of parser
    expr: Tree<TokenItem>,
    error: CASErrorKind,
}

pub(crate) fn shunting_yard(
    tokens: &Vec<TokenItem>,
    var_table: VarTable,
    args: Vec<&str>,
) -> Parsing {
    let mut output_queue: Vec<TokenItem> = vec![];
    let mut operator_stack: Vec<TokenItem> = vec![];
    let mut error = CASErrorKind::NoError;
    let mut expr: Tree<TokenItem> = Tree { root: None };

    let mut token_iter: std::iter::Peekable<std::slice::Iter<'_, TokenItem>> =
        tokens.iter().peekable();

    while let Some(token) = token_iter.next() {
        match token {
            TokenItem::Token {
                token_name,
                token_value,
            } => match token_name {
                crate::spec::TokenType::Name => {
                    // todo!();
                    let var_name: String =
                        (&<Option<Value> as Clone>::clone(&token_value).unwrap()).to_string();
                    if let Some(var) = var_table.get(&var_name) {
                        match var.args.len() {
                            0 => todo!(),
                            _ => todo!(),
                        }
                    }
                    //if predefined variable or function arg, add to output q
                    //if predefined function, add to operator q
                    //if neither, return error for undefined symbol
                }
                crate::spec::TokenType::Int => output_queue.push(token.clone()),
                crate::spec::TokenType::Float => output_queue.push(token.clone()),
                crate::spec::TokenType::Eof => break,
                crate::spec::TokenType::Assign => {
                    return Parsing {
                        expr,
                        error: CASErrorKind::AssignmentInExpression,
                    }
                }
                crate::spec::TokenType::Add => todo!(),
                crate::spec::TokenType::Sub => todo!(),
                crate::spec::TokenType::Mult => todo!(),
                crate::spec::TokenType::Div => todo!(),
                crate::spec::TokenType::Exp => todo!(),
                crate::spec::TokenType::LeftParen => operator_stack.push(token.clone()),
                crate::spec::TokenType::RightParen => todo!(),
                crate::spec::TokenType::LeftBracket => todo!(),
                crate::spec::TokenType::RightBracket => todo!(),
                crate::spec::TokenType::Comma => todo!(),
                crate::spec::TokenType::Less => todo!(),
                crate::spec::TokenType::Greater => todo!(),
                crate::spec::TokenType::Equal => todo!(),
                crate::spec::TokenType::NotEqual => todo!(),
                crate::spec::TokenType::LessEqual => todo!(),
                crate::spec::TokenType::GreaterEqual => todo!(),
                crate::spec::TokenType::Calc => todo!(),
                crate::spec::TokenType::Sim => todo!(),
                crate::spec::TokenType::Der => todo!(),
                crate::spec::TokenType::Integral => todo!(),
                crate::spec::TokenType::Const => todo!(),
                crate::spec::TokenType::ResFun => todo!(),
                crate::spec::TokenType::Error => todo!(),
            },
            TokenItem::Error { err } => {
                return Parsing { expr, error: *err };
            }
        }
    }

    return Parsing { expr, error };
}
