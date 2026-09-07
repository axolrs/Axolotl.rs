// Owner: PascalElixir / axolrs (GitHub org)
// File: Source span and location tracking for the Axolotl compiler.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A byte-offset span in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub start: u32,
    pub end: u32,
    pub file_id: u32,
}

impl Span {
    pub const DUMMY: Span = Span { start: 0, end: 0, file_id: 0 };

    /// Construct a span from start, end byte offsets and a file id.
    pub fn new(start: u32, end: u32, file_id: u32) -> Span {
        Span { start, end, file_id }
    }

    /// Return the byte length of this span.
    pub fn len(&self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    /// Return true when the span covers zero bytes.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Combine two spans into the smallest span covering both.
    pub fn union(self, other: Span) -> Span {
        if self.file_id != other.file_id {
            return self;
        }
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            file_id: self.file_id,
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Span::DUMMY
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file_id, self.start)
    }
}
