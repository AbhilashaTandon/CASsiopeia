#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::{
        parser::{shunting_yard, Parsing},
        scanner::{process_line, TokenItem},
    };
    #[test]
    fn testing() {
        let mut tokens: Vec<TokenItem> = vec![];
        process_line("2 + 2", &mut tokens, 0);

        let var_table = HashMap::new();

        let Parsing { error, .. } = shunting_yard(&tokens, var_table, vec![]);

        println! {"{:?}", error};
    }
}
