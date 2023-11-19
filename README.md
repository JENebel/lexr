[Lexr](https://github.com/JENebel/lexr-parsr/tree/master/lexr) and [parsr](https://github.com/JENebel/lexr-parsr/tree/master/parsr) are two libraries that can be used together to create a lexer and parser for a language. They can also be used separately, if you only need one of them.

They are both designed to be simple and flexible, and to be used in a variety of situations, together or not.

Currently only the [lexr](https://github.com/JENebel/lexr-parsr/tree/master/lexr) library is available, and the parsr library is in development.

Here is an example of uing the [lexr](https://github.com/JENebel/lexr-parsr/tree/master/lexr) library to create a lexer for a simple language:

```rust
use lexr::lex_rule;
#[derive(Debug, PartialEq)]
enum T {
    Integer(i32),
    Plus,
    Minus,
    Eof
}
use T::*;

lex_rule!{lex -> T {
    r"[0-9]+" => |s| Integer(s.parse().unwrap()),
    r"[+]" => |_| Plus,
    "-" => |_| Minus,
    ws => |_| continue,
    eof => |_| Eof,
}}

let tokens = lex("1 + 2 - 3").into_token_vec();
assert_eq!(tokens, vec![Integer(1), Plus, Integer(2), Minus, Integer(3), Eof]);
```