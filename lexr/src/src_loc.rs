use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A location in the source code
pub struct SrcLoc {
    abs_range: (usize, usize),
    start: (usize, usize),
    end: (usize, usize),
}

impl SrcLoc {
    /// A dummy source location. A zeroed source location.
    pub const DUMMY: Self = Self {
        start: (0, 0),
        end: (0, 0),
        abs_range: (0, 0),
    };

    /// The start of the source location. (start_line, start_col)
    pub fn start(&self) -> (usize, usize) {
        self.start
    }

    /// The end of the source location. (end_line, end_col)
    pub fn end(&self) -> (usize, usize) {
        self.end
    }

    /// Create a new source location from a start and end position
    pub fn new(start: (usize, usize), end: (usize, usize), abs: (usize, usize)) -> Self {
        Self {
            start,
            end,
            abs_range: abs,
        }
    }

    /// Combine two source locations into one, spanning from the start of the first to the end of the second
    /// 
    /// Panics if the source locations overlap
    pub fn combine(self, other: SrcLoc) -> SrcLoc {
        assert!(self.end <= other.start, "Combining overlapping source locations");
        SrcLoc {
            start: self.start,
            end: other.end,
            abs_range: (self.abs_range.0, other.abs_range.1),
        }
    }

    /// Get the source code spanned by this source location
    /// 
    /// returns (start_line, start_col, end_line, end_col)
    pub fn get_loc(&self) -> (usize, usize, usize, usize) {
        (self.start.0, self.start.1, self.end.0, self.end.1)
    }

    /// The absolute range of the source location, meaning the char indexes. [start, end)
    pub fn get_abs_loc(&self) -> (usize, usize) {
        self.abs_range
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

impl PartialOrd for SrcLoc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.start.0 == other.start.0 {
            self.start.1.partial_cmp(&other.start.1)
        } else {
            self.start.0.partial_cmp(&other.start.0)
        }
    }
}