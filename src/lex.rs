#[macro_export]
/// Define a lexer function with provided rules.
///
/// The lexer function takes a string slice and returns a vector of tokens and their locations.
/// 
/// If it is unable to parse an input, it returns an error with the first character in the unmatched subsequence, and the location of the error.
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
///     // Statics and constants are allowed
///     const WORD: &str = r"[a-zA-Z]+";
/// 
///     init_lexer!{lex, Token => 
///         r"\s+" =>         |_|  continue, // Ignore whitespace. 'continue' is the only allowed expression except for tokens and panic
///         "[0-9]+" =>       |i|  Token::Number(i.parse().unwrap()),
///         WORD =>           |id| { // You can use blocks
///                                    println!("{}", id); 
///                                    Token::Word(id.to_string()) },
///         "#" WORD "#" =>   |_|  continue, // You can use a sequence of regexes
///         "$" =>            |_|  Token::EndOfFile
///     }
///     
///     assert!(lex("123 abc #comment#").unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>() == vec![
///         Token::Number(123), 
///         Token::Word("abc".to_string()), 
///         Token::EndOfFile
///     ]);
macro_rules! init_lexer {(
        $name:ident, $token:ty => 
        $($($regex:expr)+ => |$id:pat_param| $closure:expr),* $(,)?
    ) => {
        #[allow(unreachable_code)]
        /// The lexer function
        /// 
        /// Returns a vector of tokens and their locations
        pub fn $name(input: &str) -> Result<Vec<($token, parcom::SrcLoc)>, (char, parcom::SrcLoc)> {
            let mut tokens = Vec::new();
            let mut input_iter = input.chars().peekable();
            let mut idx = 0;
            let mut line = 1;
            let mut col = 1;
            let mut empty = false;

            loop {
                // This here to allow for matching $ as eof a single time while avoiding infinite loop
                if empty { break; }
                if idx == input.len() { empty = true; }

                $(
                    let r = regex_macro::regex!({
                        let mut r_str = "^".to_string();
                        $(r_str.push_str($regex);)+
                        r_str
                    }.as_str());

                    if let Some(mat) = r.find(&input[idx..]) {
                        let length = mat.end();
                        let $id = mat.as_str();
                        
                        let mut start_line = line;
                        let mut start_col = col;
                        let mut end_line = line;
                        let mut end_col = col;

                        let end_index = idx + length - 1;
                        for i in 0..length {
                            let c = input_iter.next().unwrap();
                            if idx + i == end_index {
                                end_line = line;
                                end_col = col;
                            }
                            if c == '\n' {
                                line += 1;
                                col = 1;
                            } else {
                                col += 1;
                            }
                        }

                        idx += length;

                        let token = $closure; // If the closure is a continue, it skips the push
                        tokens.push((token, parcom::SrcLoc::new((start_line, start_col), (end_line, end_col))));
                        continue;
                    }
                )*

                if let Some(c) = input_iter.peek() {
                    return Err((*c, parcom::SrcLoc::new((line, col), (line, col))));
                }
            }

            Ok(tokens)
        }
    };
}