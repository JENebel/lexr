#[derive(Debug)]
pub struct SrcLoc {
    pub line: usize,
    pub col: usize
}

#[macro_export]
macro_rules! init_lexer {
    (type Item = $token:ty; $($regex:literal => $(_)? $(|$id:tt| $func:expr)? ;)*) => {
        pub fn lex<'a>(input: &'a str) -> Vec<($token, SrcLoc)> {
            let mut rules: Vec<(regex::Regex, Box<dyn Fn(&str) -> Option<$token>>)> = Vec::new();

            $(rules.push(init_lexer!(@lex_rule $token, $regex => $(|$id| $func)?)));*;

            let mut tokens = Vec::new();

            let mut cursor = 0;

            let mut line = 1;
            let mut col = 0;

            while cursor < input.len() {
                // Clear whitespace
                let next_char = input[cursor..].chars().next().unwrap();
                if next_char.is_whitespace() {
                    cursor += 1;

                    if next_char == '\n' {
                        col = 0;
                        line += 1;
                    } else {
                        col += 1;
                    }

                    continue;
                }

                // Find a match
                let mut matched = false;
                for (regex, func) in rules.iter() {
                    if let Some(matsch) = regex.find_at(input, cursor) {
                        if matsch.start() != cursor {
                            continue;
                        }

                        if let Some(token) = func(matsch.as_str()) {
                            tokens.push((token, SrcLoc {line, col}));
                        }

                        cursor = matsch.end();
                        col += matsch.as_str().len();
                        
                        matched = true;
                        break;
                    }
                }

                // Handle a token not found
                if !matched {
                    panic!("Unmatched syntax. at {cursor}: '{}'", &input[cursor..])
                }
            }

            tokens
        }
    };

    (@lex_rule $token:ty, $regex:literal => ) => {
        {
            (
                regex::Regex::new(&format!("{}", $regex)).expect(&format!("'{}' is not a valid regex", $regex)),
                Box::new(|_| None)
            )
        }
    };

    (@lex_rule $token:ty, $regex:literal => |$id:tt| $func:expr) => {
        {
            (
                regex::Regex::new(&format!("{}", $regex)).expect(&format!("'{}' is not a valid regex", $regex)),
                Box::new(|$id| Some($func))
            )
        }
    };
}