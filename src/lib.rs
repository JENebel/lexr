pub mod lex;
pub mod parse;

use std::fmt::Display;

pub use lex::*;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A location in the source code
pub struct SrcLoc {
    start: (usize, usize),
    end: (usize, usize),
}

impl SrcLoc {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Self {
            start,
            end,
        }
    }

    /// Combine two source locations into one, spanning from the start of the first to the end of the second
    pub fn combine(&self, other: SrcLoc) -> SrcLoc {
        assert!(self.end <= other.start, "Combining overlapping source locations");
        SrcLoc {
            start: self.start,
            end: other.end
        }
    }

    /// Get the source code spanned by this source location
    /// 
    /// returns (start_line, start_col, end_line, end_col)
    pub fn get_loc(&self) -> (usize, usize, usize, usize) {
        (self.start.0, self.start.1, self.end.0, self.end.1)
    }
}

impl Display for SrcLoc {
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

impl Default for SrcLoc {
    fn default() -> Self {
        Self {
            start: (0, 0),
            end: (0, 0),
        }
    }
}

impl PartialOrd for SrcLoc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.start.0 == other.start.0 {
            self.start.1.partial_cmp(&other.start.1)
        } else {
            self.start.0.partial_cmp(&other.start.0)
        }
    }
}