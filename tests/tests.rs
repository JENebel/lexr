#[cfg(test)]
mod tests {
    use Lit::*;
    use Token::*;
    use decl_cfg::*;

    #[derive(Debug, PartialEq)]
    pub enum Lit {
        Int(i64),
        Float(f64),
        Bool(bool),
    }

    #[derive(Debug, PartialEq)]
    pub enum Token<'a> {
        Operator(&'a str),
        Identifier(&'a str),
        Literal(Lit),
    }

    init_lexer!(
        type Item = Token;
        r"[0-9]+(\.[0-9]+)?f" => |f| Literal(Float(f[0..f.len()-1].parse().unwrap()));
        r"[0-9]+\.[0-9]+" => |f| Literal(Float(f.parse().unwrap()));
        r"[0-9]+" => |i| Literal(Int(i.parse().unwrap()));
        r"(true|false)\b" => |b| Literal(Bool(b.parse().unwrap()));
        r"[a-zA-Z_][a-zA-Z0-9_]*" => |id| Identifier(id);
        r"\+|\*|-|/|<=|==|<|%|!|\|\||\||&&|&" => |op| Operator(op);
    );

    #[test]
    fn it_works() {
        let result: Vec<Token> = lex("12.43 12 43f brian * 8 true").unwrap().into_iter().map(|(token, _)| token).collect();
        assert!(result == vec![
            Literal(Float(12.43)),
            Literal(Int(12)),
            Literal(Float(43.0)),
            Identifier("brian"),
            Operator("*"),
            Literal(Int(8)),
            Literal(Bool(true)),
        ]);
    }
}