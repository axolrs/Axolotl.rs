// Owner: PascalElixir / axolrs (GitHub org)
// File: Line index - byte-offset ↔ LSP position conversion for Gills (line + UTF-16 character units, CRLF-aware).

use tower_lsp::lsp_types::{Position, Range};

/// Maps byte offsets to LSP `line`/`character` positions for one document body.
///
/// The `character` component counts UTF-16 code units, which is the LSP default
/// and the encoding Gills advertises via `positionEncoding` in `initialize`.
/// Line breaks are `\n`; a `\r` immediately before a `\n` belongs to the
/// preceding line (CRLF files therefore index naturally).
#[derive(Debug, Clone)]
pub struct LineIndex {
    line_starts: Vec<usize>,
    len: usize,
}

impl LineIndex {
    /// Build the line index for a document body.
    pub fn new(text: &str) -> LineIndex {
        let mut line_starts = vec![0usize];
        for (i, b) in text.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        LineIndex { line_starts, len: text.len() }
    }

    /// Return the number of lines, counting the empty trailing line.
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Return the byte offset at which the given line starts (clamped).
    pub fn line_start(&self, line: u32) -> usize {
        let idx = line as usize;
        if idx >= self.line_starts.len() {
            self.len
        } else {
            self.line_starts[idx]
        }
    }

    /// Convert a byte offset into an LSP position (clamped to the text bounds).
    ///
    /// Offsets that are not char boundaries are tolerated: the position of the
    /// containing character is produced.
    pub fn position_of(&self, text: &str, offset: u32) -> Position {
        let offset = (offset as usize).min(self.len);
        let line = self.line_of(offset);
        let start = self.line_starts[line];
        let mut units = 0u32;
        for (pos, ch) in text[start..].char_indices() {
            if start + pos >= offset {
                break;
            }
            units += ch.len_utf16() as u32;
        }
        Position { line: line as u32, character: units }
    }

    /// Convert an LSP position into a byte offset (clamped to line ends and text bounds).
    ///
    /// The returned offset is always a char boundary because the walk advances
    /// character by character. Lines past the end clamp to the end of the text.
    pub fn offset_of(&self, text: &str, position: Position) -> u32 {
        let line = position.line as usize;
        if line >= self.line_starts.len() {
            return self.len as u32;
        }
        let start = self.line_starts[line];
        let end = self.line_content_end(text, line);
        let mut units = position.character;
        let mut offset = start;
        for (pos, ch) in text[start..end].char_indices() {
            if units == 0 {
                break;
            }
            let len = ch.len_utf16() as u32;
            if units < len {
                break;
            }
            units -= len;
            offset = start + pos + ch.len_utf8();
        }
        offset as u32
    }

    /// Convert a byte span into an LSP range (clamped).
    pub fn range_of(&self, text: &str, start: u32, end: u32) -> Range {
        let start = start.min(end);
        let end = end.max(start);
        Range {
            start: self.position_of(text, start),
            end: self.position_of(text, end),
        }
    }

    /// Return the line number containing the given byte offset.
    fn line_of(&self, offset: usize) -> usize {
        match self.line_starts.binary_search(&offset) {
            Ok(line) => line,
            Err(next) => next - 1,
        }
    }

    /// Return the exclusive byte offset at which the given line ends.
    fn line_end(&self, line: usize) -> usize {
        if line + 1 < self.line_starts.len() {
            self.line_starts[line + 1]
        } else {
            self.len
        }
    }

    /// Return the end of the line's content, excluding any trailing line break.
    fn line_content_end(&self, text: &str, line: usize) -> usize {
        let mut end = self.line_end(line);
        let bytes = text.as_bytes();
        if end > 0 && bytes.get(end - 1) == Some(&b'\n') {
            end -= 1;
        }
        if end > 0 && bytes.get(end - 1) == Some(&b'\r') {
            end -= 1;
        }
        end.max(self.line_starts[line])
    }
}

#[cfg(test)]
mod tests {
    use super::LineIndex;
    use tower_lsp::lsp_types::Position;

    /// Build a test position.
    fn p(line: u32, character: u32) -> Position {
        Position { line, character }
    }

    /// ASCII offsets map to line/character pairs.
    #[test]
    fn ascii_position_round_trip() {
        let text = "fn main()\n    print(1)\nend\n";
        let idx = LineIndex::new(text);
        assert_eq!(idx.position_of(text, 0), p(0, 0));
        assert_eq!(idx.position_of(text, 10), p(1, 0));
        assert_eq!(idx.position_of(text, 14), p(1, 4));
        assert_eq!(idx.position_of(text, 22), p(1, 12));
        assert_eq!(idx.position_of(text, 23), p(2, 0));
    }

    /// LSP positions map back to the byte offsets they came from.
    #[test]
    fn ascii_offset_round_trip() {
        let text = "fn main()\n    print(1)\nend\n";
        let idx = LineIndex::new(text);
        for offset in [0usize, 3, 10, 11, 15, 22, 24] {
            if offset > text.len() {
                continue;
            }
            let pos = idx.position_of(text, offset as u32);
            assert_eq!(idx.offset_of(text, pos), offset as u32);
        }
    }

    /// Multi-byte UTF-8 characters count as UTF-16 code units, not bytes.
    #[test]
    fn multibyte_counts_utf16_units() {
        let text = "let s = \"héllo→😀\"";
        let idx = LineIndex::new(text);
        let start = text.find('é').unwrap() as u32;
        let pos = idx.position_of(text, start);
        assert_eq!(pos, p(0, 10));
        let arrow = text.find('→').unwrap() as u32;
        let pos = idx.position_of(text, arrow);
        assert_eq!(pos, p(0, 14));
    }

    /// An astral-plane character counts as two UTF-16 units.
    #[test]
    fn astral_char_is_two_utf16_units() {
        let text = "a😀b";
        let idx = LineIndex::new(text);
        let pos = idx.position_of(text, text.find('b').unwrap() as u32);
        assert_eq!(pos, p(0, 3));
        assert_eq!(idx.offset_of(text, pos), 5u32);
    }

    /// Offsets past the end clamp to the end of the text.
    #[test]
    fn offset_clamps_to_end() {
        let text = "abc";
        let idx = LineIndex::new(text);
        let pos = idx.position_of(text, 99);
        assert_eq!(pos, p(0, 3));
    }

    /// Positions past the last line clamp to the end of the text.
    #[test]
    fn position_clamps_to_last_line() {
        let text = "abc\ndef";
        let idx = LineIndex::new(text);
        assert_eq!(idx.offset_of(text, p(7, 0)), 7u32);
        assert_eq!(idx.offset_of(text, p(99, 99)), 7u32);
    }

    /// Characters past the end of a line clamp to the line end.
    #[test]
    fn character_clamps_to_line_end() {
        let text = "ab\ncd";
        let idx = LineIndex::new(text);
        assert_eq!(idx.offset_of(text, p(0, 99)), 2u32);
    }

    /// CRLF line endings put the carriage return on the previous line.
    #[test]
    fn crlf_line_breaks_index_correctly() {
        let text = "one\r\ntwo\r\n";
        let idx = LineIndex::new(text);
        assert_eq!(idx.line_count(), 3);
        assert_eq!(idx.position_of(text, 0), p(0, 0));
        assert_eq!(idx.position_of(text, 5), p(1, 0));
        assert_eq!(idx.position_of(text, 3), p(0, 3));
        assert_eq!(idx.offset_of(text, p(1, 0)), 5u32);
    }

    /// Ranges convert both endpoints independently.
    #[test]
    fn range_conversion_clamps_and_orders() {
        let text = "aaaa\nbbbb";
        let idx = LineIndex::new(text);
        let r = idx.range_of(text, 2, 7);
        assert_eq!(r.start, p(0, 2));
        assert_eq!(r.end, p(1, 2));
    }

    /// Empty text has a single empty line.
    #[test]
    fn empty_text_has_one_line() {
        let idx = LineIndex::new("");
        assert_eq!(idx.line_count(), 1);
        assert_eq!(idx.line_start(0), 0);
        assert_eq!(idx.position_of("", 5), p(0, 0));
    }
}
