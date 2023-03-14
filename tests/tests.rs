#[cfg(test)]
mod tests {
    use core::panic;

    use Literal::*;
    use Token::*;
    use decl_cfg::*;
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

    pub enum Expression<'a> {
        BinOpExp(&'a Expression<'a>, Operator, &'a Expression<'a>),
        UnOpExp(Operator, &'a Expression<'a>),
        LitExp(Literal)
    }

    init_lexer!(
        TokenType = Token;

        r"[0-9]+(\.[0-9]+)?f" =>        |f| LitToken(Float(f[0..f.len()-1].parse().unwrap()));
        r"[0-9]+\.[0-9]+" =>            |f| LitToken(Float(f.parse().unwrap()));
        r"[0-9]+" =>                    |i| LitToken(Int(i.parse().unwrap()));
        r"(true|false)\b" =>            |b| LitToken(Bool(b.parse().unwrap()));
        r"[a-zA-Z_][a-zA-Z0-9_]*" =>    |id| IdToken(id);
        r"\+|\*|-|/" => |op| OpToken(Operator::from(op));
    );

    init_parser!(
        TokenType = Token;

        Exp ::= Exp, OpToken(_), Exp => |left, op, right| { BinOpExp(left, op, right) }
            | OpToken(_), Exp => |op, right| { UnOpExp(left, op, right) }
            | Lit => |lit| { LitExp(lit) }

        Block ::= OpenBrace, OneOrMoreExps, CloseBrace => |_, exps, _| { BlockExpression::new(exps) }

        OneOrMoreExps ::= Exp => |exp| { exp }
                | Exp, SemiColon, OneOrMoreExps => |exp, _, rest| { concat_exps(exp, rest) }
         
        Op  ::= OpToken(op) => |op| { op }
        Lit ::= LitToken(lit) => |lit| { lit }
    );

    #[test]
    fn it_works() {
        let result: Vec<Token> = lex("12.43 12 43f brian * 8 true").unwrap().into_iter().map(|(token, _)| token).collect();
        assert!(result == vec![
            LitToken(Float(12.43)),
            LitToken(Int(12)),
            LitToken(Float(43.0)),
            IdToken("brian"),
            OpToken(Mult),
            LitToken(Int(8)),
            LitToken(Bool(true)),
        ]);
    }

    fn bbb(exp: Expression) {
        if let Expression = exp {

        }
    }
}