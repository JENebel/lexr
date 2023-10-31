#[cfg(test)]
mod tests {
    use Literal::*;
    use Token::*;
    use parcom::*;
    use Operator::*;

    #[derive(Debug, PartialEq)]
    pub enum Literal {
        Int(i64),
        Float(f64),
        Bool(bool),
    }

    #[derive(Debug, PartialEq)]
    pub enum Token<'a> {
        OpToken(Operator),
        IdToken(&'a str),
        LitToken(Literal),
        SemiColon,
        OpenBrace,
        CloseBrace,
        Newline,
        EndOfFile,
    }

    #[derive(Debug, PartialEq)]
    pub enum Operator {
        Plus,
        Minus,
        Mult,
        Div,
    }

    impl From<& str> for Operator {
        fn from(value: & str) -> Self {
            match value {
                "+" => Self::Plus,
                "-" => Self::Minus,
                "*" => Self::Mult,
                "/" => Self::Div,
                _ => panic!("Unknown operator {value}")
            }
        }
    }

    #[test]
    fn it_works() {
        lexer!(lex -> Token {
            r"\n" => |_| Newline,
            r"\s+" => |_| continue,
            r"[0-9]+(\.[0-9]+)?f" => |f| LitToken(Float(f[0..f.len()-1].parse().unwrap())),
            r"[0-9]+\.[0-9]+" => |f| LitToken(Float(f.parse().unwrap())),
            r"[0-9]+" => |i| LitToken(Int(i.parse().unwrap())),
            r"(true|false)\b" => |b| LitToken(Bool(b.parse().unwrap())),
            r"[a-zA-Z_][a-zA-Z0-9_]*" => |id| IdToken(id),
            r"[\+\*-/]" => |op| OpToken(Operator::from(op)),
            r";" => |_| SemiColon,
            r"\{" => |_| OpenBrace,
            r"\}" => |_| CloseBrace,
            "$" => |_| EndOfFile
        });

        let prog = "12.43 12  43.0\nbrian * 8 true";
        let r = match lex(prog) {
            Ok(r) => r,
            Err(e) => panic!("Error: {} at {}", e.0, e.1)
        };
        
        let result: Vec<Token> = r.into_iter().map(|(token, l)| {println!("{}", l); token}).collect();
        //println!("{:?}", result);
        assert!(result == vec![
            LitToken(Float(12.43)),
            LitToken(Int(12)),
            LitToken(Float(43.0)),
            Newline,
            IdToken("brian"),
            OpToken(Mult),
            LitToken(Int(8)),
            LitToken(Bool(true)),
            EndOfFile
        ]);
    }

    #[test]
    fn test2() {
        const WORD: &str = r"[a-zA-Z]+";

        lexer!(lex(a: i32) -> Token {
            "#" WORD "#" => |w|  {println!("{w}:{a}"); EndOfFile},
            _ => |_| panic!()
        });

        let prog = "#hello#";

        match lex(prog, 4) {
            Ok(_) => (),
            Err(e) => panic!("Error: '{}' at {}", e.0, e.1)
        };
    }
}