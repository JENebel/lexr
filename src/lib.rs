pub mod lex;
pub mod parse;

pub use lex::*;

struct Position {
    x: i32,
    y: i32,
}

fn test(abe: i32) -> Position {
    Position {
        x: abe,
        y: abe * 2
    }
}