//parses numerical literals
//0.02, -1,304,774, 1.264e-13

fn parse_lit() {}

// fn parse_number(
//     next_char: char,
//     iter: &mut Peekable<Enumerate<str::Chars>>,
//     line_pos: &mut usize,
// ) -> Option<Result<Token, CASError>> {
//     //parses numerical literals like 3.4, 1234, -1523

//     //minus 1 since peek is following char

//     if next_char.is_numeric() || next_char == '.' {
//         let token_type = match get_next_number(next_char, iter, line_pos) {
//             //check if its a float, int, or something that cant be either
//             Ok(Token {
//                 token_type: Float(float),
//                 ..
//             }) => Float(float),
//             Ok(Token {
//                 token_type: Int(int),
//                 ..
//             }) => Int(int),

//             Err(lit) => {
//                 return Some(Err(CASError {
//                     line_pos: *line_pos,
//                     kind: CASErrorKind::MalformedNumericLiteral { lit },
//                 }))
//             }
//             _ => {
//                 return Some(Err(CASError {
//                     line_pos: *line_pos,
//                     kind: CASErrorKind::SyntaxError,
//                 }))
//             }
//         };
//         return Some(Ok(Token {
//             token_type,
//             line_pos: *line_pos,
//         }));
//     }
//     None
// }

// fn get_next_number(
//     chr: char,
//     iter: &mut Peekable<Enumerate<str::Chars>>,
//     line_pos: &mut usize,
// ) -> Result<Token, String> {
//     let mut num: String = chr.to_string();

//     while let Some(&(_, chr)) = iter.peek() {
//         if !chr.is_numeric() && chr != '.' {
//             break;
//         }
//         num.push(chr);
//         iter.next();
//         *line_pos += 1;
//     }
//     let int_parse = num.parse::<i64>();
//     if let Ok(int) = int_parse {
//         return Ok(Token {
//             token_type: Int(int.into()),
//             line_pos: *line_pos,
//         });
//     }
//     let float_parse = num.parse::<f64>();
//     match float_parse {
//         Ok(float) => {
//             return Ok(Token {
//                 token_type: Float(float),
//                 line_pos: *line_pos,
//             })
//         }
//         Err(_) => return Err(num),
//     }
// }
