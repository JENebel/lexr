use parcom::lexer;
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
    lexer!(lex -> Token {
        "Brian" => |_| Newline,
        eof => |_| EndOfFile,
    });

    let l: Vec<Token> = lex("Brian").token_vec();
    assert_eq!(l, vec![Newline, EndOfFile])
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
        eof => |_| EndOfFile
    });

    let prog = "12.43 12  6f  43.0\nbrian * 8 true";
    let lex = lex(prog);
    
    let result: Vec<Token> = lex.token_vec();
    //println!("{:?}", result);
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
        "h" => |_| LitToken(Int(1)),
        "e" => |_| LitToken(Int(2)),
        "l" => |_| LitToken(Int(3)),
        "o" => |_| LitToken(Int(4)),
        "!" => |_| break,
    }}

    for (token, loc) in lex("hel!lo") {
        println!("{}: {:?}", loc, token);
    }
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

    lexer!{lex -> Token {
        r"\s+" =>         |_|  continue, // Ignore whitespace. 'continue' is the only allowed expression except for tokens and panic
        "[0-9]+" =>       |i|  Token::Number(i.parse().unwrap()),
        WORD =>           |id| { // You can use blocks
                                println!("{}", id); 
                                Token::Word(id.to_string()) },
        "#" WORD "#" =>   |_|  continue, // You can use a sequence of regexes
        "$" =>            |_|  Token::EndOfFile
    }}

    let result: Vec<Token> = lex("123 abc #comment#").token_vec();
    assert_eq!(result, vec![
        Token::Number(123), 
        Token::Word("abc".to_string()), 
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

    lexer!{lex<'a>(s: &'a str) -> Token {
        "#" =>  |_, loc| { println!("{}", loc); continue }, // You can use a sequence of regexes
        _ =>    |_|  Token::Word(s.to_string()),
        eof =>  |_|  Token::EndOfFile,
    }}

    let result: Vec<Token> = lex("123##", "Argument").token_vec();
    assert_eq!(result, vec![
        Token::Word("Argument".to_string()),
        Token::Word("Argument".to_string()),
        Token::Word("Argument".to_string()),
        Token::EndOfFile
    ]);
}

#[test]
fn comment_test() {
    lexer!{token -> () {
        r"\s+" => |_| continue,
        r"hello" => |_| continue,
        r"/\*" => |_, _, src| { println!("{}", src.source); continue },
        r"\*/" => |_, _, src| { println!("{}", src.source); continue },
    }}

    let token = token("/* hello */").token_vec();
    assert_eq!(token, vec![]);
}