pub mod lex;
pub mod parse;

pub use lex::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SrcLoc {
    start: usize,
    end: usize
}

impl SrcLoc {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: if start < end { start} else { end },
            end
        }
    }

    pub fn combine(&self, other: SrcLoc) -> SrcLoc {
        SrcLoc {
            start: self.start,
            end: other.end,
        }
    }

    pub fn get(&self, source_code: &str) -> (usize, usize, usize, usize) {
        let mut line = 1;
        let mut col = 1;
        let mut start_line = 1;
        let mut start_col = 1;
        for (i, c) in source_code.chars().enumerate() {
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

    pub fn display(&self, source_code: &str) -> String {
        let (start_line, start_col, end_line, end_col) = self.get(source_code);
        if start_line == end_line && start_col == end_col {
            format!("{}:{}", start_line, start_col)
        } else if start_line == end_line {
            format!("{}:{}-{}", start_line, start_col, end_col)
        } else {
            format!("{}:{}-{}:{}", start_line, start_col, end_line, end_col)
        }
    }
}

impl Default for SrcLoc {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0
        }
    }
}
