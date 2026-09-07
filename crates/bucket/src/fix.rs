// Owner: PascalElixir / axolrs (GitHub org)
// File: crates/bucket/src/fix.rs - the Regrow auto-fixer: safe, mechanical fixes to .axol sources.

use crate::build::collect_axol_files;
use crate::fmt;
use crate::lint;
use crate::lint::LintLevel;
use axolc_core::span::Span;

/// The result of fixing one source string in memory.
#[derive(Debug, Clone, PartialEq)]
pub struct FixOutcome {
    pub removals: usize,
    pub source: String,
    pub parse_clean: bool,
}

/// Extend a deletion range to swallow one immediately following semicolon.
fn expand_semi(src: &str, start: usize, end: usize) -> (usize, usize) {
    let bytes = src.as_bytes();
    let mut e = end.min(bytes.len());
    while e < bytes.len() && (bytes[e] == b' ' || bytes[e] == b'\t') {
        e += 1;
    }
    if e < bytes.len() && bytes[e] == b';' {
        e += 1;
        while e < bytes.len() && (bytes[e] == b' ' || bytes[e] == b'\t') {
            e += 1;
        }
        return (start, e);
    }
    (start, end)
}

/// Delete the given spans from a source string, merging overlaps into unions.
pub fn apply_deletions(src: &str, spans: &[Span]) -> String {
    let len = src.len();
    let mut ranges: Vec<(usize, usize)> = spans
        .iter()
        .map(|s| expand_semi(src, (s.start as usize).min(len), (s.end as usize).min(len)))
        .filter(|(s, e)| s < e)
        .collect();
    ranges.sort();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (s, e) in ranges {
        if let Some(last) = merged.last_mut() {
            if s <= last.1 {
                last.1 = last.1.max(e);
                continue;
            }
        }
        merged.push((s, e));
    }
    let mut out = String::new();
    let mut pos = 0usize;
    for (s, e) in &merged {
        out.push_str(&src[pos..*s]);
        pos = *e;
    }
    out.push_str(&src[pos..len]);
    out
}

/// Fix one source string: remove side-effect-free unreachable statements and self-assignments, then run Shed.
pub fn fix_source(src: &str) -> Result<FixOutcome, String> {
    let unreachable = lint::scan_unreachable_stmts(src, 0).map_err(|e| e.message)?;
    let pure_spans: Vec<Span> = unreachable
        .iter()
        .filter(|u| u.pure)
        .map(|u| u.span)
        .collect();
    let mut cur = apply_deletions(src, &pure_spans);
    let mut removed = pure_spans.len();
    let (module, diags) = axolc_core::parse(&cur, 0);
    let parse_clean = !diags.has_errors();
    if parse_clean {
        let findings = lint::analyze(&module);
        let assign_spans: Vec<Span> = findings
            .iter()
            .filter(|f| f.code == lint::N0105)
            .map(|f| f.span)
            .collect();
        if !assign_spans.is_empty() {
            cur = apply_deletions(&cur, &assign_spans);
            removed += assign_spans.len();
        }
    }
    let source = fmt::format_source(&cur).unwrap_or(cur);
    Ok(FixOutcome { removals: removed, source, parse_clean })
}

/// Count the warning-level findings of a fixed source string.
pub fn remaining_warnings(src: &str, file_id: u32) -> usize {
    lint::lint_source(src, file_id)
        .iter()
        .filter(|f| f.level == LintLevel::Warning)
        .count()
}

/// Apply auto-fixes to the project with Regrow (the `bucket fix` command).
pub fn fix_cmd() {
    let dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let files = collect_axol_files(&dir);
    if files.is_empty() {
        println!("Regrow: no .axol files found");
        return;
    }
    let mut fixed = 0usize;
    let mut failed = 0usize;
    for f in &files {
        let src = match std::fs::read_to_string(f) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Regrow: cannot read {}: {}", f.display(), e);
                failed += 1;
                continue;
            }
        };
        match fix_source(&src) {
            Ok(outcome) => {
                if outcome.source != src {
                    if let Err(e) = std::fs::write(f, &outcome.source) {
                        eprintln!("Regrow: cannot write {}: {}", f.display(), e);
                        failed += 1;
                        continue;
                    }
                }
                fixed += outcome.removals;
            }
            Err(msg) => {
                eprintln!("{}: Regrow cannot fix: {}", f.display(), msg);
                failed += 1;
            }
        }
    }
    let mut remaining = 0usize;
    for (idx, f) in files.iter().enumerate() {
        if let Ok(src) = std::fs::read_to_string(f) {
            remaining += remaining_warnings(&src, idx as u32);
        }
    }
    println!("Regrow: fixed {} issues, {} remaining", fixed, remaining);
    if failed > 0 {
        std::process::exit(1);
    }
}
