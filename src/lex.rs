#![macro_use]
extern crate concat_idents;
#[doc(hidden)]
pub use concat_idents::*;

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
///     #[derive(PartialEq, Debug)]
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
///         eof =>            |_|  Token::EndOfFile
///     }}
///
///     let result: Vec<Token> = lex("123 abc #comment#").map(|(token, _)| token).collect();
///     assert_eq!(result, vec![
///         Token::Number(123), 
///         Token::Word("abc".to_string()), 
///         Token::EndOfFile
///     ]);
/// 
macro_rules! lexer {
    ($v:vis $name:ident $(($($arg:ident: $arg_typ:ty),*))? -> $token:ty {$($regpat:tt $($regex:expr)* => |$id:pat_param| $closure:expr),* $(,)?}) => {
    concat_idents::concat_idents!(name = _LEXER_, $name {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        /// Automatically generated lexer struct. Do not access its fields directly! Only use as iterator
        $v struct name<'a> {
            input: &'a str,
            input_iter: std::str::Chars<'a>,
            cursor: usize,
            line: usize,
            col: usize,
            empty: bool,
            $($($arg: $arg_typ),*)?
        }

        impl<'a> Iterator for name<'a> {
            type Item = ($token, parcom::SrcLoc);

            #[allow(unreachable_code)]
            fn next(&mut self) -> Option<Self::Item> {
                $($(let $arg: $arg_typ = self.$arg);*)?;

                let mut matched = false;
                loop {
                    // These allow for seamless matching of eof
                    matched = false;
                    if self.empty { break }
                    if self.input[self.cursor..].len() == 0 { self.empty = true; }
                    
                    $(
                    let regex = lexer!(@regex_rule $regpat $($regex)*);
                    if let Some(mat) = regex.find(&self.input[self.cursor..]) {
                        matched = true;
                        let length = mat.end();
                        let $id = mat.as_str();
                        
                        let start = (self.line, self.col);
                        let mut end = start;
                    
                        for i in 0..length {
                            let c = self.input_iter.next().unwrap();
                            if i == length - 1 {
                                end = (self.line, self.col);
                            }
                            if c == '\n' {
                                self.line += 1;
                                self.col = 1;
                            } else {
                                self.col += 1;
                            }
                        }

                        self.cursor += length;

                        let token = $closure;
                        return Some((token, parcom::SrcLoc::new(start, end)));
                    })*

                    break
                }

                if !self.empty && !matched {
                    if let Some(c) = self.input_iter.next() {
                        panic!("Unexpected character '{}' at {}", c, parcom::SrcLoc::new((self.line, self.col), (self.line, self.col)));
                    }
                }

                None
            }
        }

        #[doc(hidden)]
        #[doc=stringify!($token)]
        $v fn $name(input: &str $(,$($arg: $arg_typ),*)?) -> name {
            name {
                input,
                input_iter: input.chars(),
                cursor: 0,
                line: 1,
                col: 1,
                empty: false,
                $($($arg),*)?
            }
        }
    });};

    (@regex_rule _) => {
        {
            lazy_static::lazy_static! {
                static ref REGEX: regex::Regex = regex::Regex::new(r"(?s)^.").unwrap();
            }; 
            &REGEX
        }
    };

    (@regex_rule eof) => {
        {
            lazy_static::lazy_static! {
                static ref REGEX: regex::Regex = regex::Regex::new(r"^$").unwrap();
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
    };
}