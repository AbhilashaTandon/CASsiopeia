pub(crate) mod error {

    use std::string::ToString;

    #[derive(Clone, PartialEq, Debug)]
    pub(crate) enum CASErrorKind {
        NoError,
        SyntaxError,
        TypeError,
    }

    impl ToString for CASErrorKind {
        fn to_string(&self) -> String {
            return match self {
                CASErrorKind::NoError => String::from("No Error"),
                CASErrorKind::SyntaxError => String::from("Syntax Error"),
                CASErrorKind::TypeError => String::from("Type Error"),
            };
        }
    }

    impl Copy for CASErrorKind {
        todo!();
    }

    #[derive(PartialEq, Debug)]
    pub(crate) struct CASError {
        pub(crate) line_pos: usize,
        pub(crate) kind: CASErrorKind,
    }

    fn get_message(err_kind: &CASErrorKind) -> String {
        todo!();
    }

    pub(crate) fn print_error(err: CASError, line: &str, line_num: usize) {
        todo!();
    }
}
