use std::iter::Map;
use crate::SrcLoc;

/// A lexer produces tokens and locations from a defined lexing rule.
/// 
/// The lexer is an iterator over the input string, producing tokens and locations as it goes.
/// 
/// If it is unable to match any of the patterns, it will panic and report the unexpected character.
pub struct Lexer<T, Ite: Iterator<Item = (T, crate::SrcLoc)>> {
    iter: Ite,
}

impl<T, Ite: Iterator<Item = (T, crate::SrcLoc)>> Lexer<T, Ite> {
    #[doc(hidden)]
    pub fn new(iter: Ite) -> Self {
        Self { iter }
    }
}

impl<T, Ite: Iterator<Item = (T, crate::SrcLoc)>> Iterator for Lexer<T, Ite> {
    type Item = (T, SrcLoc);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T, Ite: Iterator<Item = (T, crate::SrcLoc)>> Lexer<T, Ite> {
    /// Returns an iterator over the tokens, stripping away the source locations.
    pub fn tokens(self) -> Map<Lexer<T, Ite>, impl FnMut((T, SrcLoc)) -> T> {
        self.map(|(t, _)| t)
    }

    /// Collects the tokens into a vector, stripping away the source locations.
    pub fn into_token_vec(self) -> Vec<T> {
        self.tokens().collect::<Vec<_>>()
    }

    /// Collects the token/SrcLoc pairs into a vector.
    pub fn into_vec(self) -> Vec<(T, SrcLoc)> {
        self.collect::<Vec<_>>()
    }

    /// Empties and discards all tokens in the lexer.
    pub fn exhaust(self) {
        for _ in self {}
    }

    /// Gets the next token, stripping away the SrcLoc
    pub fn next_token(&mut self) -> Option<T> {
        self.next().map(|a| a.0)
    }

    /// Consumes the lexer returning a single (token, SrcLoc) tuple.
    /// 
    /// Panics if the lexer is empty
    pub fn once(mut self) -> (T, SrcLoc) {
        match self.next() {
            Some(t) => t,
            None => panic!("Called once() on an empty lexer"),
        }
    }

    /// Consumes the lexer returning a single token.
    /// 
    /// Panics if the lexer is empty
    pub fn once_token(mut self) -> T {
        match self.next() {
            Some(t) => t.0,
            None => panic!("Called once() on an empty lexer"),
        }
    }
}