#[cfg(test)]
pub(crate) mod test {
    use scanner::{tokenize, TokenItem, Tokenization, Value};

    use crate::{scanner::*, spec::spec::TokenType};

    fn make_token(
        token_name: TokenType,
        token_text: &str,
        token_value: Option<Value>,
    ) -> TokenItem {
        return TokenItem::Token {
            token_name,
            token_text: String::from(token_text),
            token_value,
        };
    }

    #[test]
    fn variable_declarations() {
        let computed_tokens: Tokenization = tokenize(String::from("var x = 2"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(TokenType::Var, &"var", None),
            make_token(TokenType::Name, &"x", None),
            make_token(TokenType::Equal, &"=", None),
            make_token(TokenType::Int, &"int", Some(Value::Int(2))),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            error_code: 0,
        };
        assert_eq!(computed_tokens, desired_output);
    }

    #[test]
    fn function_declarations() {
        let computed_tokens: Tokenization = tokenize(String::from("fun f(x,y) = 2 * x + 3 * y"));
        let desired_tokens: Vec<TokenItem> = vec![
            make_token(TokenType::Fun, &"fun", None),
            make_token(TokenType::Name, &"f", None),
            make_token(TokenType::LeftParen, &"(", None),
            make_token(TokenType::Name, &"x", None),
            make_token(TokenType::Comma, &",", None),
            make_token(TokenType::Name, &"y", None),
            make_token(TokenType::RightParen, &")", None),
            make_token(TokenType::Equal, &"=", None),
            make_token(TokenType::Int, &"int", Some(Value::Int(2))),
            make_token(TokenType::Mult, &"*", None),
            make_token(TokenType::Name, &"x", None),
            make_token(TokenType::Add, &"+", None),
            make_token(TokenType::Int, &"int", Some(Value::Int(3))),
            make_token(TokenType::Mult, &"*", None),
            make_token(TokenType::Name, &"y", None),
        ];
        let desired_output: Tokenization = Tokenization {
            tokens: desired_tokens,
            error_code: 0,
        };
        assert_eq!(computed_tokens, desired_output);
    }
}
