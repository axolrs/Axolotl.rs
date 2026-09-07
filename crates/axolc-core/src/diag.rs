// Owner: PascalElixir / axolrs (GitHub org)
// File: Diagnostic messages produced by the Axolotl compiler front-end.

use crate::span::Span;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Severity of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

/// A single diagnostic message produced during compilation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Option<String>,
    pub message: String,
    pub span: Span,
    pub notes: Vec<String>,
}

impl Diagnostic {
    /// Construct an error diagnostic with the given message and span.
    pub fn error(message: impl Into<String>, span: Span) -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            code: None,
            message: message.into(),
            span,
            notes: Vec::new(),
        }
    }

    /// Construct a warning diagnostic with the given message and span.
    pub fn warning(message: impl Into<String>, span: Span) -> Diagnostic {
        Diagnostic {
            severity: Severity::Warning,
            code: None,
            message: message.into(),
            span,
            notes: Vec::new(),
        }
    }

    /// Attach a secondary note to the diagnostic.
    pub fn with_note(mut self, note: impl Into<String>) -> Diagnostic {
        self.notes.push(note.into());
        self
    }

    /// Attach a diagnostic code (e.g. E0001).
    pub fn with_code(mut self, code: impl Into<String>) -> Diagnostic {
        self.code = Some(code.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sev = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        };
        write!(f, "{}: {}", sev, self.message)?;
        for note in &self.notes {
            write!(f, "\n  note: {}", note)?;
        }
        Ok(())
    }
}

/// A collection of diagnostics that supports easy lookup and counting.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Diagnostics {
    pub items: Vec<Diagnostic>,
}

impl Diagnostics {
    /// Construct an empty diagnostics bag.
    pub fn new() -> Diagnostics {
        Diagnostics { items: Vec::new() }
    }

    /// Push a diagnostic onto the bag.
    pub fn push(&mut self, diag: Diagnostic) {
        self.items.push(diag);
    }

    /// Return true when the bag contains at least one error.
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Error)
    }

    /// Return the number of error diagnostics.
    pub fn error_count(&self) -> usize {
        self.items
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .count()
    }

    /// Extend this bag with another.
    pub fn extend(&mut self, other: Diagnostics) {
        self.items.extend(other.items);
    }
}
