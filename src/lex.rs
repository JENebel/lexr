#[macro_export]
/// Define a lexer function with provided rules.
///
/// Usage example:
///     
///     use parcom::init_lexer;
/// 
///     #[derive(Debug, PartialEq)]
///     pub enum Token {
///         Word(String),
///         Number(u32),
///         EndOfFile,
///     }
/// 
///     init_lexer!{lex, Token => 
///         r"[0-9]+" =>    |i|  Token::Number(i.parse().unwrap()),
///         r"[a-zA-Z]+" => |id| { println!("{}", id); Token::Word(id.to_string()) }, // You can use blocks
///         r"\s+" =>       |_|  continue, // Skip this token
///         r"$" =>         |_|  Token::EndOfFile
///     }
///     
///     assert!(lex("123 abc").unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>() == vec![
///         Token::Number(123), 
///         Token::Word("abc".to_string()), 
///         Token::EndOfFile
///     ]);
macro_rules! init_lexer {
    (
        $name:ident, $token:ty =>
        $($regex:literal => |$id:pat_param| $closure:expr),* $(,)?) => {
        #[allow(unreachable_code)]
        /// The lexer function
        /// 
        /// Returns a vector of tokens and their locations
        pub fn $name(input: &str) -> Result<Vec<($token, parcom::SrcLoc)>, (char, parcom::SrcLoc)> {
            let mut tokens = Vec::new();
            let mut input_iter = input.chars().peekable();
            let mut idx = 0;
            let mut empty = false;

            loop {
                // This here to allow for matching $ as eof a single time while avoiding infinite loop
                if empty { break; }
                if idx == input.len() { empty = true; }

                $(
                    let r = regex_macro::regex!(concat!(r"^", $regex));
                    if let Some(mat) = r.find(&input[idx..]) {
                        let length = mat.end();
                        let $id = mat.as_str();
                        
                        for _ in 0..length {
                            let _ = input_iter.next().unwrap();
                        }
                        let start_index = idx;
                        idx += length;
                        
                        let token = $closure; // If the closure is a continue, it skips the push
                        tokens.push((token, parcom::SrcLoc::new(start_index, idx - 1)));
                        continue;
                    }
                )*

                return Err((*input_iter.peek().unwrap(), parcom::SrcLoc::new(idx, idx)));
            }

            Ok(tokens)
        }
    };
}