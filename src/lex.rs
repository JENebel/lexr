pub use concat_idents::concat_idents;
pub use lazy_static;
pub use regex;

pub struct LexBuf<'a> {
    pub source: &'a str,
    pub line: usize,
    pub col: usize,
    pub empty: bool,
}

impl<'a> From<&'a str> for LexBuf<'a> {
    fn from(source: &'a str) -> Self {
        Self {
            source,
            line: 1,
            col: 1,
            empty: false,
        }
    }
}

impl<'a> From<&'a String> for LexBuf<'a> {
    fn from(source: &'a String) -> Self {
        Self {
            source,
            line: 1,
            col: 1,
            empty: false,
        }
    }
}

impl<'an> From<&Self> for LexBuf<'an> {
    fn from(buf: &Self) -> Self {
        Self {
            source: buf.source,
            line: buf.line,
            col: buf.col,
            empty: buf.empty,
        }
    }
}

#[macro_export]
/// Define a lexer function with provided rules.
///
/// The lexer function takes a string slice and returns a vector of tokens and their locations.
///
/// If it is unable to parse an input, it returns an error with the first character in the unmatched subsequence, and the location of the error.
///
/// # Examples
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
macro_rules! lex_rule {
    ($v:vis $name:ident $(<$($lt:lifetime),+>)? $(($($arg:ident: $arg_typ:ty),*))? -> $token:ty {
        $($regpat:tt $($regex:expr)* => |$id:pat_param $(,$loc_id:pat_param $(,$src_id:pat_param)?)?| $closure:expr),* $(,)?
    }) => {
    parcom::concat_idents!(name = _LEXER_, $name {
        #[allow(non_camel_case_types)]
        #[doc(hidden)]
        /// Automatically generated lexer struct. Do not access its fields directly!
        /// 
        /// The `tokens` method returns an iterator over the tokens, stripping away the source locations.
        /// 
        /// `vec` and `token_vec` methods are provided for convenience.
        $v struct name<'_src, $($($lt),+)?> {
            buf: parcom::LexBuf<'_src>,
            $($($arg: $arg_typ),*)?
        }

        impl<'_src, $($($lt),+)?> name<'_src, $($($lt),+)?> {
            #[doc(hidden)]
            /// Returns an iterator over the tokens, stripping away the source locations.
            $v fn tokens(self) -> std::iter::Map<name<'_src, $($($lt),+)?>, impl FnMut(($token, parcom::SrcLoc)) -> $token> {
                self.map(|(t, _)| t)
            }

            #[doc(hidden)]
            /// Collects the tokens in a vector, stripping away the source locations.
            $v fn token_vec(self) -> Vec<($token)> {
                self.tokens().collect::<Vec<_>>()
            }

            #[doc(hidden)]
            /// Collects the tokens in a vector.
            $v fn vec(self) -> Vec<($token, parcom::SrcLoc)> {
                self.collect::<Vec<_>>()
            }
        }

        impl<'_src, $($($lt),+)?> Iterator for name<'_src, $($($lt),+)?> {
            type Item = ($token, parcom::SrcLoc);

            #[allow(unreachable_code)]
            fn next(&mut self) -> Option<Self::Item> {
                $($(let $arg: $arg_typ = self.$arg);*)?;

                let mut matched = false;
                loop {
                    // These allow for seamless matching of eof
                    matched = false;
                    if self.buf.empty { break }
                    if self.buf.source.len() == 0 { self.buf.empty = true; }
                    
                    $(
                    let regex = lex_rule!(@regex_rule $regpat $($regex)*);
                    if let Some(mat) = regex.find(&self.buf.source) {
                        matched = true;
                        let length = mat.end();
                        
                        let start = (self.buf.line, self.buf.col);
                        let mut end = start;
                        
                        let mut source_iter = self.buf.source.chars();
                        for i in 0..length {
                            let c = source_iter.next().unwrap();
                            if i == length - 1 {
                                end = (self.buf.line, self.buf.col);
                            }
                            if c == '\n' {
                                self.buf.line += 1;
                                self.buf.col = 1;
                            } else {
                                self.buf.col += 1;
                            }
                        }
                        
                        self.buf.source = &self.buf.source[length..];

                        let $id = mat.as_str();
                        $(let $loc_id = parcom::SrcLoc::new(start, end);)?
                        $($(let $src_id = &self.buf;)?)?
                        let token = $closure;
                        return Some((token, parcom::SrcLoc::new(start, end)));
                    })*

                    break
                }

                if !self.buf.empty && !matched {
                    if let Some(c) = self.buf.source.chars().next() {
                        panic!("Unexpected character '{}' at {}", c, parcom::SrcLoc::new((self.buf.line, self.buf.col), (self.buf.line, self.buf.col)));
                    }
                }

                None
            }
        }

        #[doc(hidden)]
        /// Creates a new lexer from a string slice.
        $v fn $name<'_src $(,$($lt),+)?>(source: impl Into<parcom::LexBuf<'_src>> $(,$($arg: $arg_typ),*)?) -> name<'_src $(,$($lt),+)?> {
            name {
                buf: source.into(),
                $($($arg),*)?
            }
        }
    });};

    (@regex_rule _) => {{
        parcom::lazy_static::lazy_static! {
            static ref REGEX: parcom::regex::Regex = parcom::regex::Regex::new(r"(?s)^.").unwrap();
        }; 
        &REGEX
    }};

    (@regex_rule eof) => {{
        parcom::lazy_static::lazy_static!{
            static ref REGEX: parcom::regex::Regex = parcom::regex::Regex::new(r"^$").unwrap();
        }; 
        &REGEX
    }};

    (@regex_rule $($regex:expr)+) => {{
        parcom::lazy_static::lazy_static!{
            static ref REGEX: parcom::regex::Regex = parcom::regex::Regex::new({
                let mut r_str = "^".to_string();
                $(r_str.push_str($regex);)+
                r_str
            }.as_str()).unwrap();
        }; 
        &REGEX
    }};
}