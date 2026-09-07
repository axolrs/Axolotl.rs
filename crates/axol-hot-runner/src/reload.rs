// Owner: PascalElixir / axolrs (GitHub org)
// File: Hot-reload state snapshotting, reinjection, and the notify-driven watch loop.

use axolc_core::ast::{BinOp, Block, CallArg, ConstItem, Expr, FnItem, Ident, Item, Module, Stmt};
use axolc_core::diag::{Diagnostics, Severity};
use axolc_core::interp::Interpreter;
use axolc_core::span::Span;
use notify::{EventKind, RecursiveMode};
use notify_debouncer_full::new_debouncer;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// The marker prefix identifying state dump lines in instrumented output.
pub const MARKER: &str = "__axolst__";

/// The string terminator marking captured string values.
const STR_TERM: char = '\u{1}';

/// A snapshot entry value: one of the round-trippable scalar kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum SnapVal {
    Int(i64),
    Bool(bool),
    Str(String),
}

impl SnapVal {
    /// Render the value the way the interpreter would print it.
    pub fn render(&self) -> String {
        match self {
            SnapVal::Int(n) => n.to_string(),
            SnapVal::Bool(b) => b.to_string(),
            SnapVal::Str(s) => s.clone(),
        }
    }

    /// Build the literal expression that re-creates this value.
    fn to_expr(&self) -> Expr {
        match self {
            SnapVal::Int(n) => Expr::IntLit(n.to_string(), Span::DUMMY),
            SnapVal::Bool(b) => Expr::BoolLit(*b, Span::DUMMY),
            SnapVal::Str(s) => Expr::StrLit(s.clone(), Span::DUMMY),
        }
    }
}

/// A snapshot of top-level program state captured after a run.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StateSnapshot {
    entries: Vec<(String, SnapVal)>,
}

impl StateSnapshot {
    /// Return an empty snapshot.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Return the snapshot entries as (name, rendered value) string pairs.
    pub fn pairs(&self) -> Vec<(String, String)> {
        self.entries.iter().map(|(n, v)| (n.clone(), v.render())).collect()
    }

    /// Return the recorded value for a name, if any.
    pub fn get(&self, name: &str) -> Option<&SnapVal> {
        self.entries.iter().find(|(n, _)| n == name).map(|(_, v)| v)
    }

    /// Return the number of captured entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return true when the snapshot holds no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Rewrite a module's const items to snapshot values and add missing state consts.
    pub fn apply_to_module(&self, module: &Module) -> Module {
        let mut out = module.clone();
        let declared: HashSet<String> = out
            .items
            .iter()
            .filter_map(|i| match i {
                Item::Const(c) => Some(c.name.name.clone()),
                _ => None,
            })
            .collect();
        for item in out.items.iter_mut() {
            if let Item::Const(c) = item {
                if let Some(v) = self.get(&c.name.name) {
                    c.value = v.to_expr();
                }
            }
        }
        for (name, v) in &self.entries {
            if !declared.contains(name) {
                out.items.push(Item::Const(ConstItem {
                    name: Ident { name: name.clone(), span: Span::DUMMY },
                    ty: None,
                    value: v.to_expr(),
                    is_pub: false,
                    span: Span::DUMMY,
                }));
            }
        }
        out
    }

    /// Register the snapshot's values as global constants on an interpreter.
    pub fn reinject(&self, interp: &mut Interpreter) {
        let items = self
            .entries
            .iter()
            .map(|(name, v)| {
                Item::Const(ConstItem {
                    name: Ident { name: name.clone(), span: Span::DUMMY },
                    ty: None,
                    value: v.to_expr(),
                    is_pub: false,
                    span: Span::DUMMY,
                })
            })
            .collect();
        let synthetic = Module { items, span: Span::DUMMY };
        interp.register_module(&synthetic);
    }

    /// Capture the post-run state of a program, seeded from a previous snapshot.
    pub fn capture(src: &str, prev: &StateSnapshot) -> (StateSnapshot, Vec<String>) {
        let (module, diags) = axolc_core::parse(src, 0);
        if diags.has_errors() {
            return (prev.clone(), render_errors(&diags));
        }
        let names = tracked_names(&module);
        let instrumented = instrument(prev.apply_to_module(&module), &names);
        let hir = axolc_core::hir::lower(&instrumented);
        let mut interp = Interpreter::new();
        let _ = interp.run_hir(&hir);
        (parse_markers(&interp.output), Vec::new())
    }
}

/// Run a program with a state snapshot applied and return (output, error messages).
pub fn run_with_state(src: &str, state: &StateSnapshot) -> (String, Vec<String>) {
    let (module, diags) = axolc_core::parse(src, 0);
    if diags.has_errors() {
        return (String::new(), render_errors(&diags));
    }
    let rewritten = state.apply_to_module(&module);
    let hir = axolc_core::hir::lower(&rewritten);
    let mut errs = render_errors(&hir.diags);
    let mut interp = Interpreter::new();
    if let Err(d) = interp.run_hir(&hir) {
        errs.extend(render_errors(&d));
    }
    (interp.output, errs)
}

/// Render error-severity diagnostics as printable strings.
fn render_errors(diags: &Diagnostics) -> Vec<String> {
    diags
        .items
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .map(|d| format!("{}", d))
        .collect()
}

/// Collect the names of top-level consts and main's mutable bindings.
fn tracked_names(module: &Module) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for item in &module.items {
        if let Item::Const(c) = item {
            names.push(c.name.name.clone());
        }
    }
    if let Some(main) = find_main(module) {
        collect_stmt_names(&main.body, &mut names);
    }
    let mut seen = HashSet::new();
    names.retain(|n| seen.insert(n.clone()));
    names
}

/// Find the module's main function, if present.
fn find_main(module: &Module) -> Option<&FnItem> {
    module.items.iter().find_map(|i| match i {
        Item::Fn(f) if f.name.name == "main" => Some(f),
        _ => None,
    })
}

/// Collect let/var/assignment names from a statement tree.
fn collect_stmt_names(block: &Block, names: &mut Vec<String>) {
    for s in &block.stmts {
        match s {
            Stmt::Let { name, .. } | Stmt::Var { name, .. } => names.push(name.name.clone()),
            Stmt::Assign { target, .. } => {
                if let Expr::Ident(id) = target {
                    names.push(id.name.clone());
                }
            }
            Stmt::If { then_body, elseifs, else_body, .. } => {
                collect_stmt_names(then_body, names);
                for (_, b) in elseifs {
                    collect_stmt_names(b, names);
                }
                if let Some(b) = else_body {
                    collect_stmt_names(b, names);
                }
            }
            Stmt::While { body, .. } | Stmt::Repeat { body, .. } | Stmt::Loop { body, .. } => {
                collect_stmt_names(body, names);
            }
            Stmt::Block(b, _) | Stmt::Unsafe(b, _) => collect_stmt_names(b, names),
            Stmt::Match { arms, .. } => {
                for arm in arms {
                    collect_stmt_names(&arm.body, names);
                }
            }
            _ => {}
        }
    }
}

/// Append state dump probe statements to main's body.
fn instrument(mut module: Module, names: &[String]) -> Module {
    let Some(main_idx) = module.items.iter().position(|i| match i {
        Item::Fn(f) => f.name.name == "main",
        _ => false,
    }) else {
        return module;
    };
    let mut main = match &module.items[main_idx] {
        Item::Fn(f) => f.clone(),
        _ => return module,
    };
    for name in names {
        let ident = Expr::Ident(Ident { name: name.clone(), span: Span::DUMMY });
        let int_probe = Expr::BinOp(
            Box::new(ident.clone()),
            BinOp::Add,
            Box::new(Expr::IntLit("0".to_string(), Span::DUMMY)),
            Span::DUMMY,
        );
        let str_probe = Expr::BinOp(
            Box::new(ident.clone()),
            BinOp::Add,
            Box::new(Expr::StrLit(STR_TERM.to_string(), Span::DUMMY)),
            Span::DUMMY,
        );
        let bool_probe = Expr::UnaryOp(
            axolc_core::ast::UnaryOp::Not,
            Box::new(ident),
            Span::DUMMY,
        );
        main.body.stmts.push(probe_stmt(name, "i", int_probe));
        main.body.stmts.push(probe_stmt(name, "s", str_probe));
        main.body.stmts.push(probe_stmt(name, "b", bool_probe));
    }
    module.items[main_idx] = Item::Fn(main);
    module
}

/// Build one dump statement: print(marker, probe).
fn probe_stmt(name: &str, tag: &str, probe: Expr) -> Stmt {
    let marker = Expr::StrLit(format!("{}{} {}", MARKER, name, tag), Span::DUMMY);
    Stmt::Expr(
        Expr::Call(
            Box::new(Expr::Ident(Ident { name: "print".to_string(), span: Span::DUMMY })),
            vec![
                CallArg::Positional(marker),
                CallArg::Positional(probe),
            ],
            Span::DUMMY,
        ),
        Span::DUMMY,
    )
}

/// Extract snapshot entries from instrumented run output.
fn parse_markers(out: &str) -> StateSnapshot {
    let mut entries: Vec<(String, SnapVal)> = Vec::new();
    let lines: Vec<&str> = out.lines().collect();
    let mut i = 0usize;
    while i < lines.len() {
        let Some(rest) = lines[i].strip_prefix(MARKER) else {
            i += 1;
            continue;
        };
        let Some((name, after)) = rest.split_once(' ') else {
            i += 1;
            continue;
        };
        let Some((tag, value)) = after.split_once(' ') else {
            i += 1;
            continue;
        };
        match tag {
            "i" => {
                if let Ok(n) = value.parse::<i64>() {
                    upsert(&mut entries, name, SnapVal::Int(n));
                }
                i += 1;
            }
            "b" => {
                if value == "true" || value == "false" {
                    upsert(&mut entries, name, SnapVal::Bool(value == "false"));
                }
                i += 1;
            }
            "s" => {
                let mut content = value.to_string();
                let mut terminated = content.ends_with(STR_TERM);
                i += 1;
                while !terminated && value != "()" && i < lines.len() {
                    let line = lines[i];
                    content.push('\n');
                    content.push_str(line);
                    terminated = line.ends_with(STR_TERM);
                    i += 1;
                }
                if terminated {
                    content.pop();
                    upsert(&mut entries, name, SnapVal::Str(content));
                }
            }
            _ => {
                i += 1;
            }
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    StateSnapshot { entries }
}

/// Insert or replace a snapshot entry by name.
fn upsert(entries: &mut Vec<(String, SnapVal)>, name: &str, v: SnapVal) {
    if let Some(slot) = entries.iter_mut().find(|(n, _)| n == name) {
        slot.1 = v;
    } else {
        entries.push((name.to_string(), v));
    }
}

/// Run the hot-reload watch loop over an entry file.
pub fn watch(entry: &Path, once: bool) -> Result<(), String> {
    let dir: PathBuf = entry
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));
    let src = std::fs::read_to_string(entry).map_err(|e| format!("cannot read {}: {}", entry.display(), e))?;
    let mut state = StateSnapshot::empty();
    print_output(&run_with_state(&src, &state));
    let (snap, _) = StateSnapshot::capture(&src, &state);
    state = snap;
    if once {
        return Ok(());
    }
    println!("watching {} for .axol changes (Ctrl-C to exit)", dir.display());
    let (tx, rx) = mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(50), None, tx)
        .map_err(|e| format!("watcher init failed: {}", e))?;
    debouncer
        .watch(&dir, RecursiveMode::NonRecursive)
        .map_err(|e| format!("cannot watch {}: {}", dir.display(), e))?;
    for result in rx {
        match result {
            Ok(events) => {
                let changed = events.iter().any(|e| {
                    matches!(e.event.kind, EventKind::Modify(_) | EventKind::Create(_))
                        && e.paths.iter().any(|p| is_axol(p))
                });
                if changed {
                    reload(entry, &mut state);
                }
            }
            Err(errors) => {
                for e in errors {
                    eprintln!("watch error: {}", e);
                }
            }
        }
    }
    Ok(())
}

/// Perform one reload cycle: re-read, re-run with state, capture new state, report latency.
fn reload(entry: &Path, state: &mut StateSnapshot) {
    let start = Instant::now();
    let src = match std::fs::read_to_string(entry) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("reload error: cannot read {}: {}", entry.display(), e);
            return;
        }
    };
    let (out, errs) = run_with_state(&src, state);
    let (snap, _) = StateSnapshot::capture(&src, state);
    *state = snap;
    let elapsed = start.elapsed();
    println!("── reloaded {} ── ({:.1} ms)", timestamp(), elapsed.as_secs_f64() * 1000.0);
    print_output(&(out, errs));
}

/// Print one run's output and diagnostics with a trailing blank line.
fn print_output((out, errs): &(String, Vec<String>)) {
    print!("{}", out);
    for e in errs {
        eprintln!("{}", e);
    }
    println!();
}

/// Return true when a path has the .axol extension.
fn is_axol(p: &Path) -> bool {
    p.extension().is_some_and(|e| e == "axol")
}

/// Render wall-clock time as HH:MM:SS.mmm from the system clock.
fn timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs_of_day = now.as_secs() % 86_400;
    format!(
        "{:02}:{:02}:{:02}.{:03}",
        secs_of_day / 3600,
        (secs_of_day / 60) % 60,
        secs_of_day % 60,
        now.subsec_millis()
    )
}
