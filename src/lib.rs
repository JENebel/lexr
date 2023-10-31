pub mod lex;
pub mod parse;

use std::fmt::Display;

pub use lex::*;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A location in the source code
pub struct SrcLoc<'a> {
    start: usize,
    end: usize,
    source: &'a str
}

impl<'a> SrcLoc<'a> {
    pub fn new(start: usize, end: usize, source: &'a str) -> Self {
        Self {
            start: if start < end { start} else { end },
            end,
            source
        }
    }

    /// Combine two source locations into one, spanning from the start of the first to the end of the second
    pub fn combine(&self, other: SrcLoc) -> SrcLoc {
        assert!(self.source == other.source, "Combining source locations from different sources");
        assert!(self.end <= other.start, "Combining overlapping source locations");
        SrcLoc {
            start: self.start,
            end: other.end,
            source: self.source
        }
    }

    /// Get the source code spanned by this source location
    /// 
    /// returns (start_line, start_col, end_line, end_col)
    pub fn get_loc(&self) -> (usize, usize, usize, usize) {
        let mut line = 1;
        let mut col = 1;
        let mut start_line = 1;
        let mut start_col = 1;
        for (i, c) in self.source.chars().enumerate() {
            if i == self.start {
                start_line = line;
                start_col = col;
            }
            if i == self.end {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (start_line, start_col, line, col)
    }
}

impl<'a> Display for SrcLoc<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start_line, start_col, end_line, end_col) = self.get_loc();
        if start_line == end_line && start_col == end_col {
            write!(f, "{}:{}", start_line, start_col)
        } else if start_line == end_line {
            write!(f, "{}:{}-{}", start_line, start_col, end_col)
        } else {
            write!(f, "{}:{}-{}:{}", start_line, start_col, end_line, end_col)
        }
    }
}

impl<'a> Default for SrcLoc<'a> {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            source: ""
        }
    }
}
