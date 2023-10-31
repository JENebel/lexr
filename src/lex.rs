#[macro_export]
/// Define a lexer function with provided rules.
///
/// The lexer function takes a string slice and returns a vector of tokens and their locations.
/// 
/// If it is unable to parse an input, it returns an error with the first character in the unmatched subsequence, and the location of the error.
/// 
/// Usage example:
///     
///     use parcom::lexer;
/// 
///     #[derive(Debug, PartialEq)]
///     pub enum Token {
///         Word(String),
///         Number(u32),
///         EndOfFile,
///     }
/// 
///     // Statics and constants can be used to reuse regexes
///     const WORD: &str = r"[a-zA-Z]+";
/// 
///     lexer!{lex -> Token {
///         r"\s+" =>         |_|  continue, // Ignore whitespace. 'continue' is the only allowed expression except for tokens and panic
///         "[0-9]+" =>       |i|  Token::Number(i.parse().unwrap()),
///         WORD =>           |id| { // You can use blocks
///                                    println!("{}", id); 
///                                    Token::Word(id.to_string()) },
///         "#" WORD "#" =>   |_|  continue, // You can use a sequence of regexes
///         "$" =>            |_|  Token::EndOfFile
///     }}
///     
///     assert!(lex("123 abc #comment#").unwrap().into_iter().map(|(t, _)| t).collect::<Vec<_>>() == vec![
///         Token::Number(123), 
///         Token::Word("abc".to_string()), 
///         Token::EndOfFile
///     ]);
macro_rules! lexer {
    ($name:ident $(($($arg:ident: $arg_typ:ty),*))? -> $token:ty {$($regpat:tt $($regex:expr)* => |$id:pat_param| $closure:expr),* $(,)?}) => {
        #[allow(unreachable_code)]
        /// The lexer function
        /// 
        /// Returns a vector of tokens and their locations
        pub fn $name(input: &str $(,$($arg: $arg_typ),*)?) -> Result<Vec<($token, parcom::SrcLoc)>, (char, parcom::SrcLoc)> {
            let mut tokens = Vec::new();
            let mut input_iter = input.chars();
            let mut idx = 0;
            let (mut line, mut col) = (1, 1);
            let mut empty = false;

            loop {
                // This here to allow for matching $ as eof a single time while avoiding infinite loop
                if empty { break; }
                if idx == input.len() { empty = true; }

                $(
                    let re = lexer!(@regex_rule $regpat $($regex)*);

                    if let Some(mat) = re.find(&input[idx..]) {
                        let length = mat.end();
                        let $id = mat.as_str();
                        
                        let start = (line, col);
                        let mut end = (line, col);
                    
                        for i in 0..length {
                            let c = input_iter.next().unwrap();
                            if i == length - 1 {
                                end = (line, col);
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
                        tokens.push((token, parcom::SrcLoc::new(start.min(end), end)));
                        continue;
                    }
                )*

                if let Some(c) = input_iter.next() {
                    return Err((c, parcom::SrcLoc::new((line, col), (line, col))));
                }
            }

            Ok(tokens)
        }
    };

    (@regex_rule _) => {
        {
            lazy_static::lazy_static! {
                static ref REGEX: regex::Regex = regex::Regex::new("(?s).").unwrap();
            }; 
            &REGEX
        }
    };

    (@regex_rule $($regex:expr)+) => {
        {
            lazy_static::lazy_static! {
                static ref REGEX: regex::Regex = regex::Regex::new({
                    let mut r_str = "^".to_string();
                    $(r_str.push_str($regex);)+
                    r_str
                }.as_str()).unwrap();
            }; 
            &REGEX
        }
    }
}