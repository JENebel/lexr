use parcom::lex_rule;
use Literal::*;
use Token::*;
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
    Dummy,
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
fn test1() {
    lex_rule!(lex -> Token {
        "Brian" => |_| Newline,
        eof => |_| EndOfFile,
    });

    let l: Vec<Token> = lex("Brian".into()).into_token_vec();
    assert_eq!(l, vec![Newline, EndOfFile])
}

#[test]
fn it_works() {
    const INT: &str = r"[0-9]+";
    const FLOAT: &str = r"[0-9]+\.([0-9]+)?";
    lex_rule!(lex -> Token {
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
        eof => |_| EndOfFile
    });

    let prog = "12.43 12  6f  43.0\nbrian * 8 true";
    let buf = prog.into();
    let lex = lex(buf);
    
    let result: Vec<Token> = lex.into_token_vec();
    assert_eq!(result, vec![
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

    lex_rule!(lex() -> Token {
        "#" WORD "#" => |_| EndOfFile,
        
    });

    let prog = "#hello#";

    let buf = prog.into();
    let l = lex(buf);

    let result: Vec<Token> = l.into_token_vec();
    assert_eq!(result, vec![EndOfFile]);
}

#[test]
fn test3() {
    lex_rule!{lex -> Token {
        "h" => |_| LitToken(Int(1)),
        "e" => |_| LitToken(Int(2)),
        "l" => |_| LitToken(Int(3)),
        "o" => |_| LitToken(Int(4)),
        "!" => |_| break,
    }}

    let tokens = lex("hello!".into()).into_token_vec();
    assert_eq!(tokens, vec![
        LitToken(Int(1)),
        LitToken(Int(2)),
        LitToken(Int(3)),
        LitToken(Int(3)),
        LitToken(Int(4)),
    ]);
}

#[test]
fn test4() {
    #[derive(PartialEq, Debug)]
    pub enum Token {
        Word(String),
        Number(u32),
        EndOfFile,
    }

    // Statics and constants can be used to reuse regexes
    const WORD: &str = r"[a-zA-Z]+";

    lex_rule!{lex -> Token {
        r"\s+" =>         |_|  continue, // Ignore whitespace. 'continue' is the only allowed expression except for tokens and panic
        "[0-9]+" =>       |i|  Token::Number(i.parse().unwrap()),
        WORD =>           |id| { // You can use blocks
                                let id = format!("{}!", id); 
                                Token::Word(id) },
        "#" WORD "#" =>   |_|  continue, // You can use a sequence of regexes
        "$" =>            |_|  Token::EndOfFile
    }}

    let result: Vec<Token> = lex("123 abc #comment#".into()).into_token_vec();
    assert_eq!(result, vec![
        Token::Number(123), 
        Token::Word("abc!".to_string()), 
        Token::EndOfFile
    ]);
}

#[test]
fn test5() {
    #[derive(PartialEq, Debug)]
    pub enum Token {
        Word(String),
        EndOfFile,
    }

    lex_rule!{lex<'a>(s: &'a str) -> Token {
        "#" =>  |_| continue, // You can use a sequence of regexes
        _ =>    |_|  Token::Word(s.to_string()),
        eof =>  |_|  Token::EndOfFile,
    }}

    let result: Vec<Token> = lex("123##".into(), "Argument").into_token_vec();
    assert_eq!(result, vec![
        Token::Word("Argument".to_string()),
        Token::Word("Argument".to_string()),
        Token::Word("Argument".to_string()),
        Token::EndOfFile
    ]);
}

#[test]
fn comment_test() {
    lex_rule!{lexer -> Token {
        r"\s+" => |_| continue,
        "h" => |_| LitToken(Int(1)),
        "e" => |_| LitToken(Int(2)),
        "l" => |_| LitToken(Int(3)),
        "o" => |_| LitToken(Int(4)),
        r"\(\*" => |_, _, src| { comment(src, 0).empty(); continue },
        r"\*\)" => |_, _, _| continue,
    }}

    lex_rule!{comment(depth: u32) -> () {
        r"\(\*" => |_, _, src| { comment(src, depth + 1).empty(); continue },
        r"\*\)" => |_, _, src| if depth == 0 { break } else { comment(src, depth - 1).empty(); continue },
        _ => |_| continue,
    }}

    let lexr = lexer("(* hello *)hello".into());
    
    

    let token = lexr.into_token_vec();
    assert_eq!(token, vec![
        LitToken(Int(1)),
        LitToken(Int(2)),
        LitToken(Int(3)),
        LitToken(Int(3)),
        LitToken(Int(4)),
    ]);
}

#[test]
fn test_sub_rules() {
    lex_rule!{lexer -> Token {
        r"-" => |_| Dummy,
        r"<" => |_, _, src| { sub(src).empty(); continue },
        eof =>  |_| EndOfFile,
    }}

    lex_rule!{sub() -> Token {
        r">" => |_, _, _| break,
        r"-" => |_| continue,
    }}

    let token = lexer("--<-->--".into()).into_token_vec();
    assert_eq!(token, vec![
        Dummy,
        Dummy,
        Dummy,
        Dummy,
        EndOfFile
    ]);
}