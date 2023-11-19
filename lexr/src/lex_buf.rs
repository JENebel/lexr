use std::{rc::Rc, cell::RefCell};

/// A buffer for lexing.
/// 
/// This is a wrapper around a string slice that stores the current state of the lexer.
/// 
/// Lexing functions are provided with a buffer,
/// which is normally automatically generated from an input string.
/// Thus you would rarely need to use this struct directly.
pub struct LexBuf<'a> {
    pub source: Rc<RefCell<&'a str>>,
    pub line: Rc<RefCell<usize>>,
    pub col: Rc<RefCell<usize>>,
    pub idx: Rc<RefCell<usize>>,
    pub empty: Rc<RefCell<bool>>,
}

impl<'a> LexBuf<'a> {
    pub fn share(&self) -> Self {
        Self {
            source: self.source.clone(),
            line: self.line.clone(),
            col: self.col.clone(),
            idx: self.idx.clone(),
            empty: self.empty.clone(),
        }
    }
}

impl<'a> LexBuf<'a> {
    pub fn new(source: impl Into<LexBuf<'a>>) -> Self {
        source.into()
    }
}

impl<'a> From<&'a str> for LexBuf<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            source: Rc::new(RefCell::new(value)),
            line: Rc::new(RefCell::new(1)),
            col: Rc::new(RefCell::new(1)),
            idx: Rc::new(RefCell::new(0)),
            empty: Rc::new(RefCell::new(false)),
        }
    }
}

impl<'a> From<&'a String> for LexBuf<'a> {
    fn from(source: &'a String) -> Self {
        Self {
            source: Rc::new(RefCell::new(source)),
            line: Rc::new(RefCell::new(1)),
            col: Rc::new(RefCell::new(1)),
            idx: Rc::new(RefCell::new(0)),
            empty: Rc::new(RefCell::new(false)),
        }
    }
}