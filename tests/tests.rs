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
    pub enum Token {
        OpToken(Operator),
        IdToken(String),
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
        const INT: &str = r"[0-9]+";
        const FLOAT: &str = r"[0-9]+\.([0-9]+)?";
        lexer!(lex -> Token {
            r"\n" => |_| Newline,
            r"\s+" => |_| continue,
            "[" INT "|" FLOAT "]f" => |f| LitToken(Float(f[0..f.len() - 1].parse().unwrap())),
            FLOAT => |f| LitToken(Float(f.parse().unwrap())),
            INT => |i| LitToken(Int(i.parse().unwrap())),
            r"(true|false)\b" => |b| LitToken(Bool(b.parse().unwrap())),
            r"[a-zA-Z_][a-zA-Z0-9_]*" => |id| IdToken(id.to_string()),
            r"[\+\*-/]" => |op| OpToken(Operator::from(op)),
            r";" => |_| SemiColon,
            r"\{" => |_| OpenBrace,
            r"\}" => |_| CloseBrace,
            "$" => |_| EndOfFile
        });

        let prog = "12.43 12  6f  43.0\nbrian * 8 true";
        let lex = lex(prog);
        
        let result: Vec<Token> = lex.map(|(token, _)| token).collect();
        println!("{:?}", result);
        //println!("{:?}", result);
        assert!(result == vec![
            LitToken(Float(12.43)),
            LitToken(Int(12)),
            LitToken(Float(6.0)),
            LitToken(Float(43.0)),
            Newline,
            IdToken("brian".to_string()),
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
            
        });

        let prog = "#hello#";

        let l = lex(prog, 5);
        for l in l {
            println!("{:?}", l);
        }
    }

    #[test]
    fn test3() {
        lexer!{lex -> Token {
            "h" => |_| Token::LitToken(Int(1)),
            "e" => |_| Token::LitToken(Int(2)),
            "l" => |_| Token::LitToken(Int(3)),
            "o" => |_| Token::LitToken(Int(4)),
            "!" => |_| continue,
        }}

        for (token, loc) in lex("hel!lo") {
            println!("{}: {:?}", loc, token);
        }
    }
}