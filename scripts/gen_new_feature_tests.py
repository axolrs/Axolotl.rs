#!/usr/bin/env python3
"""Generate new-feature test files for axolc-core (structs, methods, ranges,
repeat, ownership, new codegen). Emits deterministic tests with real assertions
into crates/axolc-core/tests/ and prints an assertion/fn count summary."""

import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT_DIR = os.path.join(ROOT, "crates", "axolc-core", "tests")

HEADER = "// Owner: PascalElixir / axolrs (GitHub org)\n// File: {desc}\n"


def raw(src: str) -> str:
    """Wrap an Axolotl source in a Rust raw string literal."""
    assert '"#' not in src
    return 'r#"' + src + '"#'


class File:
    def __init__(self, desc: str):
        self.desc = desc
        self.parts = []
        self.fns = 0

    def fn(self, name: str, body: str):
        self.parts.append("#[test]\nfn %s() {\n%s}\n" % (name, body))
        self.fns += 1

    def interp_test(self, name: str, src: str, expect: str):
        """Interpreter test asserting clean diagnostics and exact output."""
        self.fn(name, "    let (out, diags) = interpret(%s, 0);\n"
                      "    assert!(!diags.has_errors(), \"%s: expected no errors, got: {:?}\", diags.items);\n"
                      "    assert_eq!(out, %s);\n" % (raw(src), name, raw(expect)))

    def render(self) -> str:
        return HEADER.format(desc=self.desc) + "\n" + "\n".join(self.parts)


def emit_output(fileobj: File):
    path = os.path.join(OUT_DIR, fileobj.name)
    with open(path, "w") as f:
        f.write(fileobj.render())
    body = open(path).read()
    n_assert = len(re.findall(r"\bassert", body))
    n_fn = len(re.findall(r"#\[test\]", body))
    print("%-28s fns=%-5d assertions=%-5d bytes=%d" % (fileobj.name, n_fn, n_assert, len(body)))
    return n_fn, n_assert


# ---------------------------------------------------------------------------
# gen_structs.rs
# ---------------------------------------------------------------------------

def gen_structs():
    f = File("Auto-generated tests for struct declarations, literals, field reads, field mutation (single and nested), field defaults, and struct display.")
    f.name = "gen_structs.rs"
    f.parts.append("use axolc_core::interpret;\nuse axolc_core::parse;\nuse axolc_core::compile_to_rust;\n")

    # 1. struct declaration parsing (name + field count + defaults)
    for i in range(1, 21):
        nf = 2 + (i % 4)
        fields = "".join("    f%d: Int = %d\n" % (j, j * i) for j in range(1, nf + 1))
        src = "S%d = struct\n%send" % (i, fields)
        f.fn(
            "struct_decl_fields_%d" % i,
            "    let (module, diags) = parse(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert_eq!(module.items.len(), 1);\n"
            "    let mut found = 0;\n"
            "    for item in &module.items {\n"
            "        if let axolc_core::ast::Item::Struct(s) = item {\n"
            "            assert_eq!(s.name.name, \"S%d\");\n"
            "            assert_eq!(s.fields.len(), %d);\n"
            "            found += 1;\n"
            "        }\n"
            "    }\n"
            "    assert_eq!(found, 1);\n" % (raw(src), i, nf),
        )

    # 2. struct literal field reads
    for i in range(1, 41):
        x = i * 3
        y = i * 7 - 2
        src = "fn main() do\n  let p = P { x = %d, y = %d }\n  print(p.x)\n  print(p.y)\n  print(p.x + p.y)\nend" % (x, y)
        f.interp_test("struct_read_xy_%d" % i, src, "%d\n%d\n%d\n" % (x, y, x + y))

    # 3. positional struct constructor (declaration order)
    for i in range(1, 26):
        x = 10 + i
        y = 90 - i
        src = ("Vec = struct\n  x: Int = 0\n  y: Int = 0\nend\n\n"
               "fn main() do\n  let v = Vec(%d, %d)\n  print(v.x)\n  print(v.y)\n  print(v.x * v.y)\nend") % (x, y)
        f.interp_test("struct_ctor_positional_%d" % i, src, "%d\n%d\n%d\n" % (x, y, x * y))

    # 4. single field mutation
    for i in range(1, 31):
        a = i * 5
        b = 2 * i + 1
        src = "fn main() do\n  let p = P { x = %d }\n  p.x = %d\n  print(p.x)\n  p.x = p.x + %d\n  print(p.x)\nend" % (a, b, i)
        f.interp_test("struct_mut_single_%d" % i, src, "%d\n%d\n" % (b, b + i))

    # 5. compound field assign
    for i in range(1, 21):
        a = i * 4
        d = 3
        src = ("fn main() do\n  let p = P { x = %d, y = %d }\n  p.x += %d\n  p.y -= %d\n"
               "  p.x *= 2\n  print(p.x)\n  print(p.y)\nend") % (a, a + 1, d, 2)
        f.interp_test("struct_mut_compound_%d" % i, src, "%d\n%d\n" % ((a + d) * 2, a + 1 - 2))

    # 6. nested field mutation
    for i in range(1, 31):
        xi = i * 2
        yi = i * 3
        nx = 40 + i
        ny = 80 - i
        src = ("Pos = struct\n  x: Int = 0\n  y: Int = 0\nend\n\n"
               "Particle = struct\n  pos: Pos = Pos { x = 0, y = 0 }\n  mass: Int = 1\nend\n\n"
               "fn main() do\n"
               "  let p = Particle { pos = Pos { x = %d, y = %d }, mass = %d }\n"
               "  print(p.pos.x)\n  print(p.pos.y)\n"
               "  p.pos.x = %d\n  print(p.pos.x)\n"
               "  p.pos.y += %d\n  print(p.pos.y)\n"
               "  print(p.mass)\nend") % (xi, yi, i, nx, i)
        f.interp_test("struct_mut_nested_%d" % i, src,
                      "%d\n%d\n%d\n%d\n%d\n" % (xi, yi, nx, yi + i, i))

    # 7. struct display: single field (deterministic)
    for i in range(1, 21):
        v = i * 11
        src = "fn main() do\n  let s = Solo { v = %d }\n  print(s)\nend" % v
        f.interp_test("struct_display_single_%d" % i, src, "Solo {v = %d}\n" % v)

    # 8. struct display: multi field (contains both entries)
    for i in range(1, 16):
        x = i + 100
        y = i * 2 + 1
        src = "fn main() do\n  let p = P { x = %d, y = %d }\n  print(p)\nend" % (x, y)
        f.fn(
            "struct_display_multi_%d" % i,
            "    let (out, diags) = interpret(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert_eq!(out.lines().count(), 1);\n"
            "    assert!(out.contains(\"P {\"));\n"
            "    assert!(out.contains(\"x = %d\"));\n"
            "    assert!(out.contains(\"y = %d\"));\n" % (raw(src), x, y),
        )

    # 9. struct field defaults are parsed
    for i in range(1, 16):
        has_def = i % 2 == 1
        decl = "D%d = struct\n  a: Int%s\n  b: String%s\nend" % (
            i, " = %d" % (i * 9) if has_def else "", " = \"s\"" if i % 3 == 0 else "")
        src = decl + "\n\nfn main() do\n  print(1)\nend"
        f.fn(
            "struct_default_parse_%d" % i,
            "    let (module, diags) = parse(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    let s = module.items.iter().find_map(|it| match it {\n"
            "        axolc_core::ast::Item::Struct(s) if s.name.name == \"D%d\" => Some(s),\n"
            "        _ => None,\n"
            "    });\n"
            "    let s = s.unwrap();\n"
            "    assert_eq!(s.fields.len(), 2);\n"
            "    assert_eq!(s.fields[0].default.is_some(), %s);\n"
            "    assert_eq!(s.fields[1].default.is_some(), %s);\n" % (
                raw(src), i, "true" if has_def else "false", "true" if i % 3 == 0 else "false"),
        )

    # 10. struct codegen
    for i in range(1, 21):
        v = i * 6
        src = ("P = struct\n  x: Int = 0\n  name: String = \"n\"\nend\n\n"
               "fn main() do\n  let p = P { x = %d, name = \"ax%d\" }\n  print(p.x)\nend") % (v, i)
        f.fn(
            "struct_codegen_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"struct P {\"));\n"
            "    assert!(rust.contains(\"x: i64,\"));\n"
            "    assert!(rust.contains(\"name: String,\"));\n"
            "    assert!(rust.contains(\"P { x: %d, name: \\\"ax%d\\\".to_string() }\"));\n" % (
                raw(src), v, i),
        )

    # 11. empty struct literal emits ::default()
    for i in range(1, 11):
        src = ("E = struct\n  v: Int = %d\nend\n\nfn main() do\n  let e = E { }\n  print(2)\nend") % i
        f.fn(
            "struct_codegen_empty_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"struct E {\"));\n"
            "    assert!(rust.contains(\"E::default()\"));\n" % raw(src),
        )

    # 12. struct field read through expression values
    for i in range(1, 21):
        a = i + 2
        b = i * i
        src = ("fn main() do\n  let p = P { x = %d + %d, y = %d * 2 }\n"
               "  print(p.x)\n  print(p.y)\n  print(p.x - p.y)\nend") % (a, b, a)
        f.interp_test("struct_read_expr_%d" % i, src, "%d\n%d\n%d\n" % (a + b, a * 2, a + b - a * 2))

    # 13. struct equality of fields via comparison
    for i in range(1, 16):
        v = i * 3
        src = ("fn main() do\n  let a = P { x = %d }\n  let b = P { x = %d }\n"
               "  if a.x == b.x then print(\"eq\") else print(\"ne\") end\nend") % (v, v)
        f.interp_test("struct_field_eq_%d" % i, src, "eq\n")

    return f


# ---------------------------------------------------------------------------
# gen_methods.rs
# ---------------------------------------------------------------------------

def gen_methods():
    f = File("Auto-generated tests for method definitions (both syntaxes), colon and dot calls, self mutation with caller write-back, methods returning values, methods with multiple params, and static-method codegen.")
    f.name = "gen_methods.rs"
    f.parts.append("use axolc_core::interpret;\nuse axolc_core::compile_to_rust;\n")

    # 1. `Type.method = fn(self, ...) do` syntax + colon call with write-back
    for i in range(1, 31):
        x0 = i
        d = i * 10
        r = x0 + d
        src = ("M = struct\n  v: Int = 0\nend\n\n"
               "M.bump = fn(self, by) do\n  self.v = self.v + by\n  return self.v\nend\n\n"
               "fn main() do\n  let m = M { v = %d }\n  let r = m:bump(%d)\n  print(r)\n  print(m.v)\nend") % (x0, d)
        f.interp_test("method_eqfn_colon_%d" % i, src, "%d\n%d\n" % (r, r))

    # 2. `Type.method(self, ...) do` block syntax + dot call with write-back
    for i in range(1, 31):
        x0 = i * 2
        d = i
        r = x0 - d
        src = ("M = struct\n  v: Int = 0\nend\n\n"
               "M.drop(self, by) do\n  self.v = self.v - by\n  return self.v\nend\n\n"
               "fn main() do\n  let m = M { v = %d }\n  let r = m.drop(%d)\n  print(r)\n  print(m.v)\nend") % (x0, d)
        f.interp_test("method_block_dot_%d" % i, src, "%d\n%d\n" % (r, r))

    # 3. methods returning values (multiple params)
    for i in range(1, 26):
        x0 = i
        a = 3
        b = 4
        src = ("M = struct\n  v: Int = 0\nend\n\n"
               "M.mix = fn(self, a, b) do\n  return self.v * 10 + a * 2 + b\nend\n\n"
               "fn main() do\n  let m = M { v = %d }\n  print(m:mix(%d, %d))\n  print(m.v)\nend") % (x0, a, b)
        f.interp_test("method_return_multi_%d" % i, src, "%d\n%d\n" % (x0 * 10 + a * 2 + b, x0))

    # 4. self mutation write-back accumulates across repeated calls
    for i in range(1, 21):
        n = 3 + (i % 5)
        by = 2
        src = ("C = struct\n  n: Int = 0\nend\n\n"
               "C.inc = fn(self) do\n  self.n = self.n + %d\n  return self.n\nend\n\n"
               "fn main() do\n  let c = C { n = 0 }\n  let i = 0\n"
               "  while i < %d do\n    c:inc()\n    i = i + 1\n  end\n  print(c.n)\nend") % (by, n)
        f.interp_test("method_writeback_loop_%d" % i, src, "%d\n" % (n * by))

    # 5. methods combining several self fields
    for i in range(1, 21):
        a = i
        b = i * 3
        src = ("P = struct\n  x: Int = 0\n  y: Int = 0\nend\n\n"
               "P.area = fn(self, k) do\n  return (self.x + self.y) * k\nend\n\n"
               "fn main() do\n  let p = P { x = %d, y = %d }\n  print(p:area(2))\n  print(p:area(3))\nend") % (a, b)
        f.interp_test("method_read_fields_%d" % i, src, "%d\n%d\n" % ((a + b) * 2, (a + b) * 3))

    # 6. method mutating one field, others preserved
    for i in range(1, 21):
        a = i
        b = 7 * i
        src = ("P = struct\n  x: Int = 0\n  y: Int = 0\nend\n\n"
               "P.setx = fn(self, v) do\n  self.x = v\n  return v\nend\n\n"
               "fn main() do\n  let p = P { x = 0, y = %d }\n  p:setx(%d)\n  print(p.x)\n  print(p.y)\nend") % (b, a)
        f.interp_test("method_preserves_fields_%d" % i, src, "%d\n%d\n" % (a, b))

    # 7. colon and dot call equivalence on the same object
    for i in range(1, 21):
        v = i * 5
        src = ("D = struct\n  v: Int = 0\nend\n\n"
               "D.double = fn(self) do\n  self.v = self.v * 2\n  return self.v\nend\n\n"
               "fn main() do\n  let a = D { v = %d }\n  let b = D { v = %d }\n"
               "  let r1 = a:double()\n  let r2 = b.double()\n"
               "  print(r1)\n  print(r2)\n  print(a.v)\n  print(b.v)\nend") % (v, v)
        f.interp_test("method_colon_dot_equiv_%d" % i, src, "%d\n%d\n%d\n%d\n" % (v * 2, v * 2, v * 2, v * 2))

    # 8. methods with compound self-assign
    for i in range(1, 16):
        v = i
        src = ("C = struct\n  n: Int = 0\nend\n\n"
               "C.add = fn(self, d) do\n  self.n += d\n  return self.n\nend\n\n"
               "fn main() do\n  let c = C { n = %d }\n  c:add(10)\n  c:add(5)\n  print(c.n)\nend") % v
        f.interp_test("method_compound_self_%d" % i, src, "%d\n" % (v + 15))

    # 9. method calls inside for loops
    for i in range(1, 16):
        n = 4 + (i % 4)
        src = ("C = struct\n  n: Int = 0\nend\n\n"
               "C.inc = fn(self) do\n  self.n = self.n + 1\n  return self.n\nend\n\n"
               "fn main() do\n  let c = C { n = 0 }\n  for k in %d do\n    c:inc()\n  end\n  print(c.n)\nend") % n
        f.interp_test("method_in_for_%d" % i, src, "%d\n" % n)

    # 10. codegen: impl grouping, &mut self, &self, static call shape
    for i in range(1, 21):
        mult = 1 + i
        src = ("P = struct\n  x: Int = 0\n  y: Int = 0\nend\n\n"
               "P.scale = fn(self, k) do\n  self.x = self.x * k\n  return self.x\nend\n\n"
               "P.getsum = fn(self) do\n  return self.x + self.y\nend\n\n"
               "P.origin = fn(x0) do\n  return P { x = x0, y = 0 }\nend\n\n"
               "fn main() do\n  let p = P { x = %d, y = 2 }\n  let a = p:scale(%d)\n"
               "  let b = p.getsum()\n  let o = P:origin(7)\n  print(a)\nend") % (i, mult)
        f.fn(
            "method_codegen_impl_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"impl P {\"));\n"
            "    assert_eq!(rust.matches(\"impl P {\").count(), 1);\n"
            "    assert!(rust.contains(\"fn scale(&mut self, k)\"));\n"
            "    assert!(rust.contains(\"fn getsum(&self)\"));\n"
            "    assert!(rust.contains(\"P::origin(7)\"));\n" % raw(src),
        )

    # 11. two separate method decls group into a single impl block
    for i in range(1, 11):
        src = ("T%d = struct\n  v: Int = 0\nend\n\n"
               "T%d.a = fn(self) do\n  return self.v\nend\n\n"
               "T%d.b = fn(self) do\n  self.v = self.v + 1\n  return self.v\nend\n\n"
               "fn main() do\n  let t = T%d { v = 1 }\n  print(t:b())\nend") % (i, i, i, i)
        f.fn(
            "method_codegen_grouped_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"impl T%d {\"));\n"
            "    assert_eq!(rust.matches(\"impl T%d {\").count(), 1);\n"
            "    assert!(rust.contains(\"fn a(&self)\"));\n"
            "    assert!(rust.contains(\"fn b(&mut self)\"));\n" % (raw(src), i, i),
        )

    return f


# ---------------------------------------------------------------------------
# gen_ranges.rs
# ---------------------------------------------------------------------------

def gen_ranges():
    f = File("Auto-generated tests for numeric for loops (the lowered Range form), loop bounds, break/continue, nested loops, HIR range lowering, and range codegen.")
    f.name = "gen_ranges.rs"
    f.parts.append("use axolc_core::interpret;\nuse axolc_core::compile_to_rust;\nuse axolc_core::lower;\nuse axolc_core::ast::{Expr, Stmt};\nuse axolc_core::hir::Item;\n")

    # 1. numeric for sums: for i in N -> iterates 0..N-1
    for i in range(1, 41):
        n = 3 + i
        total = n * (n - 1) // 2
        src = "fn main() do\n  var s = 0\n  for i in %d do\n    s = s + i\n  end\n  print(s)\nend" % n
        f.interp_test("range_for_sum_%d" % i, src, "%d\n" % total)

    # 2. numeric for prints each index
    for i in range(1, 21):
        n = 2 + (i % 5)
        expect = "".join("%d\n" % k for k in range(n))
        src = "fn main() do\n  for i in %d do\n    print(i)\n  end\nend" % n
        f.interp_test("range_for_print_%d" % i, src, expect)

    # 3. zero-iteration loop
    for i in range(1, 11):
        src = "fn main() do\n  var s = %d\n  for i in 0 do\n    s = s + 1\n  end\n  print(s)\nend" % i
        f.interp_test("range_for_zero_%d" % i, src, "%d\n" % i)

    # 4. nested numeric for
    for i in range(1, 21):
        n = 3 + (i % 4)
        m = 2 + (i % 3)
        total = n * m * (m - 1) // 2 + n * (n - 1) // 2
        src = ("fn main() do\n  var s = 0\n  for i in %d do\n    for j in %d do\n"
               "      s = s + j\n    end\n    s = s + i\n  end\n  print(s)\nend") % (n, m)
        f.interp_test("range_for_nested_%d" % i, src, "%d\n" % total)

    # 5. break in range loop
    for i in range(1, 21):
        n = 8
        k = 2 + (i % 5)
        total = k * (k - 1) // 2
        src = ("fn main() do\n  var s = 0\n  for i in %d do\n    if i == %d then break end\n"
               "    s = s + i\n  end\n  print(s)\nend") % (n, k)
        f.interp_test("range_for_break_%d" % i, src, "%d\n" % total)

    # 6. continue in range loop
    for i in range(1, 21):
        n = 7
        k = 2 + (i % 4)
        total = n * (n - 1) // 2 - k
        src = ("fn main() do\n  var s = 0\n  for i in %d do\n    if i == %d then continue end\n"
               "    s = s + i\n  end\n  print(s)\nend") % (n, k)
        f.interp_test("range_for_continue_%d" % i, src, "%d\n" % total)

    # 7. expression bounds (evaluated at runtime by the interpreter)
    for i in range(1, 11):
        a = 2
        b = 1 + (i % 3)
        n = a + b
        total = n * (n - 1) // 2
        src = "fn main() do\n  var s = 0\n  for i in (%d + %d) do\n    s = s + i\n  end\n  print(s)\nend" % (a, b)
        f.interp_test("range_for_expr_bound_%d" % i, src, "%d\n" % total)

    # 8. HIR lowering: for i in N becomes Range(0, N, false)
    for i in range(1, 21):
        n = 5 + i
        src = "fn main() do\n  for i in %d do\n    print(i)\n  end\nend" % n
        f.fn(
            "range_lowering_%d" % i,
            "    let (module, diags) = lower(%s, 0);\n"
            "    assert!(diags.items.iter().all(|d| d.severity != axolc_core::diag::Severity::Error));\n"
            "    let mut ranges = 0;\n"
            "    for item in &module.items {\n"
            "        if let Item::Fn(fun) = item {\n"
            "            for s in &fun.body.stmts {\n"
            "                if let Stmt::For { iter, .. } = s {\n"
            "                    if let Expr::Range(lo, hi, inc, _) = iter {\n"
            "                        assert_eq!(*inc, false);\n"
            "                        assert!(matches!(lo.as_deref(), Some(Expr::IntLit(t, _)) if t == \"0\"));\n"
            "                        assert!(matches!(hi.as_deref(), Some(Expr::IntLit(t, _)) if t == \"%d\"));\n"
            "                        ranges += 1;\n"
            "                    }\n"
            "                }\n"
            "            }\n"
            "        }\n"
            "    }\n"
            "    assert_eq!(ranges, 1);\n" % (raw(src), n),
        )

    # 9. codegen emits `for i in 0..N`
    for i in range(1, 21):
        n = 4 + i
        src = "fn main() do\n  var s = 0\n  for i in %d do\n    s = s + i\n  end\n  print(s)\nend" % n
        f.fn(
            "range_codegen_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"for i in 0..%d {\"));\n"
            "    assert!(rust.contains(\"println!\"));\n" % (raw(src), n),
        )

    # 10. for-in over array literals
    for i in range(1, 21):
        arr = [i, i + 1, i + 2, i + 3]
        total = sum(arr)
        src = "fn main() do\n  var s = 0\n  for x in [%s] do\n    s = s + x\n  end\n  print(s)\nend" % (
            ", ".join(str(v) for v in arr))
        f.interp_test("range_for_array_%d" % i, src, "%d\n" % total)

    return f


# ---------------------------------------------------------------------------
# gen_repeat.rs
# ---------------------------------------------------------------------------

def gen_repeat():
    f = File("Auto-generated tests for repeat/until semantics (body runs at least once, correct exit), nested repeat, break inside repeat, and repeat codegen.")
    f.name = "gen_repeat.rs"
    f.parts.append("use axolc_core::interpret;\nuse axolc_core::compile_to_rust;\n")

    # 1. body runs exactly once when condition is initially true
    for i in range(1, 26):
        start = i
        src = "fn main() do\n  var c = %d\n  repeat\n    c = c + 1\n  until true\n  print(c)\nend" % start
        f.interp_test("repeat_once_%d" % i, src, "%d\n" % (start + 1))

    # 2. counts until a threshold
    for i in range(1, 31):
        n = 2 + i
        src = "fn main() do\n  var c = 0\n  repeat\n    c = c + 1\n  until c >= %d\n  print(c)\nend" % n
        f.interp_test("repeat_count_%d" % i, src, "%d\n" % n)

    # 3. repeat accumulates values
    for i in range(1, 21):
        n = 3 + (i % 6)
        total = n * (n + 1) // 2
        src = ("fn main() do\n  var c = 0\n  var s = 0\n  repeat\n    c = c + 1\n    s = s + c\n"
               "  until c >= %d\n  print(c)\n  print(s)\nend") % n
        f.interp_test("repeat_accumulate_%d" % i, src, "%d\n%d\n" % (n, total))

    # 4. nested repeat
    for i in range(1, 21):
        outer = 2 + (i % 3)
        inner = 3 + (i % 4)
        src = ("fn main() do\n  var i = 0\n  var hits = 0\n"
               "  repeat\n    var j = 0\n"
               "    repeat\n      j = j + 1\n      hits = hits + 1\n"
               "    until j >= %d\n"
               "    i = i + 1\n"
               "  until i >= %d\n"
               "  print(i)\n  print(j)\n  print(hits)\nend") % (inner, outer)
        f.interp_test("repeat_nested_%d" % i, src, "%d\n%d\n%d\n" % (outer, inner, outer * inner))

    # 5. break exits the innermost repeat
    for i in range(1, 16):
        stop = 2 + (i % 4)
        src = ("fn main() do\n  var c = 0\n  repeat\n    c = c + 1\n"
               "    if c == %d then break end\n  until false\n  print(c)\nend") % stop
        f.interp_test("repeat_break_%d" % i, src, "%d\n" % stop)

    # 6. compound condition with arithmetic
    for i in range(1, 16):
        n = i * 2
        src = "fn main() do\n  var c = 0\n  repeat\n    c = c + %d\n  until c * c >= 100\n  print(c)\nend" % n
        c = 0
        while c * c < 100:
            c += n
        f.interp_test("repeat_arith_cond_%d" % i, src, "%d\n" % c)

    # 7. repeat with while-like countdown
    for i in range(1, 16):
        start = 5 + i
        src = ("fn main() do\n  var n = %d\n  var steps = 0\n  repeat\n    n = n - 1\n    steps = steps + 1\n"
               "  until n == 0\n  print(n)\n  print(steps)\nend") % start
        f.interp_test("repeat_countdown_%d" % i, src, "0\n%d\n" % start)

    # 8. codegen: loop + if + break
    for i in range(1, 16):
        n = 3 + i
        src = "fn main() do\n  var c = 0\n  repeat\n    c = c + 1\n  until c >= %d\n  print(c)\nend" % n
        f.fn(
            "repeat_codegen_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"loop {\"));\n"
            "    assert!(rust.contains(\"if c >= %d {\"));\n"
            "    assert!(rust.contains(\"break;\"));\n"
            "    assert!(!rust.contains(\"while\"));\n" % (raw(src), n),
        )

    return f


# ---------------------------------------------------------------------------
# gen_ownership_tests.rs
# ---------------------------------------------------------------------------

def gen_ownership():
    f = File("Auto-generated tests for ownership inference: E0101 use-after-move, E0102 move-out-of-borrow, E0103 dangling return, E0104 write-through-borrow, parameter modes, advisory hints, move sites, and clean programs.")
    f.name = "gen_ownership_tests.rs"
    f.parts.append(
        "use axolc_core::infer_ownership;\nuse axolc_core::ownership::ParamMode;\n\n"
        "/// Collect the diagnostic codes produced by ownership inference for a source.\n"
        "fn own_codes(src: &str) -> Vec<String> {\n"
        "    let (_, diags) = infer_ownership(src, 0);\n"
        "    diags.items.iter().filter_map(|d| d.code.clone()).collect()\n"
        "}\n\n"
        "/// Count error-severity diagnostics from ownership inference.\n"
        "fn own_errors(src: &str) -> usize {\n"
        "    let (_, diags) = infer_ownership(src, 0);\n"
        "    diags.error_count()\n"
        "}\n\n"
        "/// Resolve one parameter mode.\n"
        "fn own_mode(src: &str, fname: &str, pname: &str) -> Option<ParamMode> {\n"
        "    let (table, _) = infer_ownership(src, 0);\n"
        "    table.param_mode(fname, pname)\n"
        "}\n\n"
        "/// Collect the advisory hints for one function.\n"
        "fn own_hints(src: &str, fname: &str) -> Vec<(String, ParamMode)> {\n"
        "    let (table, _) = infer_ownership(src, 0);\n"
        "    table.hints_for(fname)\n"
        "}\n\n"
        "/// Resolve all parameter modes for one function.\n"
        "fn own_modes(src: &str, fname: &str) -> Vec<(String, ParamMode)> {\n"
        "    let (table, _) = infer_ownership(src, 0);\n"
        "    table.modes_for(fname)\n"
        "}\n\n"
        "/// Collect move sites (function, moved name, callee).\n"
        "fn own_moves(src: &str) -> Vec<(String, String, String)> {\n"
        "    let (table, _) = infer_ownership(src, 0);\n"
        "    table.moves.iter().map(|(f, m)| (f.clone(), m.name.clone(), m.callee.clone())).collect()\n"
        "}\n"
    )

    # 1. E0101 use-after-move fires (use in tail after the move call)
    for i in range(1, 26):
        v = i
        src = ("fn take(p: move P) do\n  print(p.x)\nend\n\n"
               "fn main() do\n  let a = P { x = %d }\n  take(a)\n  print(a.x)\nend") % v
        f.fn(
            "own_e0101_fires_%d" % i,
            "    let codes = own_codes(%s);\n"
            "    assert!(codes.iter().any(|c| c == \"E0101\"), \"expected E0101, got {:?}\", codes);\n"
            "    assert!(own_errors(%s) >= 1);\n" % (raw(src), raw(src)),
        )

    # 2. E0101 does NOT fire when the value is used before the move
    for i in range(1, 16):
        v = i
        src = ("fn take(p: move P) do\n  print(p.x)\nend\n\n"
               "fn main() do\n  let a = P { x = %d }\n  print(a.x)\n  take(a)\n  print(1)\nend") % v
        f.fn(
            "own_e0101_not_before_%d" % i,
            "    let codes = own_codes(%s);\n"
            "    assert!(!codes.iter().any(|c| c == \"E0101\"), \"unexpected E0101, got {:?}\", codes);\n" % raw(src),
        )

    # 3. E0102 move-out-of-borrow
    for i in range(1, 16):
        src = ("fn take(p: move P) do\n  print(p.x)\nend\n\n"
               "fn f(p: &P) do\n  take(p)\n  print(1)\nend")
        f.fn(
            "own_e0102_fires_%d" % i,
            "    let codes = own_codes(%s);\n"
            "    assert!(codes.iter().any(|c| c == \"E0102\"), \"expected E0102, got {:?}\", codes);\n" % raw(src),
        )

    # 4. E0103 dangling return
    for i in range(1, 21):
        v = i + 3
        src = ("fn f() -> &Int do\n  let n = %d\n  return n\nend") % v
        f.fn(
            "own_e0103_fires_%d" % i,
            "    let codes = own_codes(%s);\n"
            "    assert!(codes.iter().any(|c| c == \"E0103\"), \"expected E0103, got {:?}\", codes);\n" % raw(src),
        )

    # 5. E0103 does not fire for by-value returns
    for i in range(1, 11):
        v = i
        src = "fn f() -> Int do\n  let n = %d\n  return n\nend" % v
        f.fn(
            "own_e0103_value_ok_%d" % i,
            "    let codes = own_codes(%s);\n"
            "    assert!(!codes.iter().any(|c| c == \"E0103\"), \"unexpected E0103, got {:?}\", codes);\n" % raw(src),
        )

    # 6. E0104 write through borrow
    for i in range(1, 21):
        v = i
        src = "fn f(p: &P) do\n  p.x = %d\n  print(p.x)\nend" % v
        f.fn(
            "own_e0104_fires_%d" % i,
            "    let codes = own_codes(%s);\n"
            "    assert!(codes.iter().any(|c| c == \"E0104\"), \"expected E0104, got {:?}\", codes);\n" % raw(src),
        )

    # 7. E0104 does not fire through a mutable borrow
    for i in range(1, 11):
        v = i
        src = "fn f(p: &var P) do\n  p.x = %d\n  print(p.x)\nend" % v
        f.fn(
            "own_e0104_mut_ok_%d" % i,
            "    let codes = own_codes(%s);\n"
            "    assert!(!codes.iter().any(|c| c == \"E0104\"), \"unexpected E0104, got {:?}\", codes);\n" % raw(src),
        )

    # 8. parameter modes
    for i in range(1, 21):
        src = ("fn reader(p: P) do\n  print(p.x)\nend\n\n"
               "fn writer(p: P) do\n  p.x = %d\n  print(p.x)\nend\n\n"
               "fn mover(p: move P) do\n  print(p.x)\nend\n\n"
               "fn valuer(n: Int) do\n  print(n)\nend\n\n"
               "fn valuewriter(n: Int) do\n  n = %d\n  print(n)\nend") % (i, i)
        f.fn(
            "own_modes_%d" % i,
            "    let src = %s;\n"
            "    assert_eq!(own_mode(src, \"reader\", \"p\"), Some(ParamMode::Borrow));\n"
            "    assert_eq!(own_mode(src, \"writer\", \"p\"), Some(ParamMode::BorrowMut));\n"
            "    assert_eq!(own_mode(src, \"mover\", \"p\"), Some(ParamMode::Move));\n"
            "    assert_eq!(own_mode(src, \"valuer\", \"n\"), Some(ParamMode::Value));\n"
            "    assert_eq!(own_mode(src, \"valuewriter\", \"n\"), Some(ParamMode::Value));\n"
            "    assert_eq!(own_mode(src, \"reader\", \"n\"), None);\n" % raw(src),
        )

    # 9. explicit borrow annotations resolve
    for i in range(1, 11):
        src = ("fn ro(p: &P) do\n  print(p.x)\nend\n\n"
               "fn rw(p: &var P) do\n  p.x = %d\nend") % i
        f.fn(
            "own_modes_borrow_%d" % i,
            "    let src = %s;\n"
            "    assert_eq!(own_mode(src, \"ro\", \"p\"), Some(ParamMode::Borrow));\n"
            "    assert_eq!(own_mode(src, \"rw\", \"p\"), Some(ParamMode::BorrowMut));\n" % raw(src),
        )

    # 10. hints are advisory (inferred from usage, ignoring explicit annotations)
    for i in range(1, 11):
        src = ("fn mover(p: move P) do\n  print(p.x)\nend\n\n"
               "fn writer(p: P) do\n  p.x = %d\nend") % i
        f.fn(
            "own_hints_%d" % i,
            "    let src = %s;\n"
            "    assert_eq!(own_hints(src, \"mover\"), vec![(\"p\".to_string(), ParamMode::Borrow)]);\n"
            "    assert_eq!(own_hints(src, \"writer\"), vec![(\"p\".to_string(), ParamMode::BorrowMut)]);\n"
            "    assert_eq!(own_mode(src, \"mover\", \"p\"), Some(ParamMode::Move));\n" % raw(src),
        )

    # 11. modes_for lists every parameter in order
    for i in range(1, 11):
        src = ("fn three(a: Int, b: P, c: move P) do\n  print(a)\n  print(b.x)\n  print(c.x)\nend")
        f.fn(
            "own_modes_for_%d" % i,
            "    let src = %s;\n"
            "    let modes = own_modes(src, \"three\");\n"
            "    assert_eq!(modes.len(), 3);\n"
            "    assert_eq!(modes[0], (\"a\".to_string(), ParamMode::Value));\n"
            "    assert_eq!(modes[1], (\"b\".to_string(), ParamMode::Borrow));\n"
            "    assert_eq!(modes[2], (\"c\".to_string(), ParamMode::Move));\n"
            "    assert!(own_modes(src, \"missing\").is_empty());\n" % raw(src),
        )

    # 12. move sites recorded
    for i in range(1, 11):
        v = i * 2
        src = ("fn take(p: move P) do\n  print(p.x)\nend\n\n"
               "fn main() do\n  let a = P { x = %d }\n  take(a)\n  print(1)\nend") % v
        f.fn(
            "own_moves_%d" % i,
            "    let moves = own_moves(%s);\n"
            "    assert_eq!(moves.len(), 1);\n"
            "    assert_eq!(moves[0], (\"main\".to_string(), \"a\".to_string(), \"take\".to_string()));\n" % raw(src),
        )

    # 13. clean programs produce no ownership errors
    clean_sources = [
        "fn main() do\n  var s = 0\n  for i in 10 do\n    s = s + i\n  end\n  print(s)\nend",
        "fn add(a: Int, b: Int) do\n  return a + b\nend\n\nfn main() do\n  print(add(2, 3))\nend",
        "P = struct\n  x: Int = 0\nend\n\nfn main() do\n  let p = P { x = 1 }\n  print(p.x)\nend",
        "fn main() do\n  var c = 0\n  repeat\n    c = c + 1\n  until c >= 3\n  print(c)\nend",
        "fn main() do\n  let a = [1, 2, 3]\n  print(a[1])\nend",
        "fn main() do\n  match 5\n    1 => print(\"one\")\n    5 => print(\"five\")\n    _ => print(\"other\")\n  end\nend",
    ]
    for i, src in enumerate(clean_sources * 3, 1):
        f.fn(
            "own_clean_%d" % i,
            "    assert_eq!(own_errors(%s), 0);\n"
            "    assert!(own_codes(%s).is_empty());\n" % (raw(src), raw(src)),
        )

    # 14. multiple moves in one function
    for i in range(1, 11):
        v = i
        src = ("fn take(p: move P) do\n  print(p.x)\nend\n\n"
               "fn main() do\n  let a = P { x = %d }\n  let b = P { x = %d }\n  take(a)\n  take(b)\n  print(1)\nend") % (v, v + 1)
        f.fn(
            "own_moves_two_%d" % i,
            "    let moves = own_moves(%s);\n"
            "    assert_eq!(moves.len(), 2);\n"
            "    assert!(moves.iter().any(|m| m.1 == \"a\"));\n"
            "    assert!(moves.iter().any(|m| m.1 == \"b\"));\n" % raw(src),
        )

    return f


# ---------------------------------------------------------------------------
# gen_codegen_new.rs
# ---------------------------------------------------------------------------

def gen_codegen_new():
    f = File("Auto-generated tests for new codegen shapes: impl grouping, &mut self, Type::static() calls, println!, for-range, loop+break from repeat, .to_string() for String fields, let mut inference, pipe calls, struct literals, and FFI extraction for cblocks.")
    f.name = "gen_codegen_new.rs"
    f.parts.append("use axolc_core::compile_to_rust;\nuse axolc_core::ffi;\n")

    # 1. impl blocks with &mut self
    for i in range(1, 16):
        k = 1 + i
        src = ("W = struct\n  v: Int = 0\nend\n\n"
               "W.grow = fn(self, k) do\n  self.v = self.v * k\n  return self.v\nend\n\n"
               "fn main() do\n  let w = W { v = %d }\n  print(w:grow(%d))\nend") % (i, k)
        f.fn(
            "cg_mut_self_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"impl W {\"));\n"
            "    assert!(rust.contains(\"fn grow(&mut self, k)\"));\n" % raw(src),
        )

    # 2. &self for read-only methods
    for i in range(1, 16):
        src = ("R = struct\n  v: Int = 0\nend\n\n"
               "R.get = fn(self) do\n  return self.v\nend\n\n"
               "fn main() do\n  let r = R { v = %d }\n  print(r:get())\nend") % i
        f.fn(
            "cg_ref_self_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"fn get(&self)\"));\n"
            "    assert!(!rust.contains(\"fn get(&mut self)\"));\n" % raw(src),
        )

    # 3. Type::static() call emission
    for i in range(1, 16):
        v = i * 3
        src = ("S = struct\n  v: Int = 0\nend\n\n"
               "S.make = fn(v) do\n  return S { v = v }\nend\n\n"
               "fn main() do\n  let s = S:make(%d)\n  print(s.v)\nend") % v
        f.fn(
            "cg_static_call_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"S::make(%d)\"));\n"
            "    assert!(!rust.contains(\"let s = S.make\"));\n" % (raw(src), v),
        )

    # 4. println! emission for print
    for i in range(1, 16):
        v = i * 4
        src = "fn main() do\n  print(%d)\n  print(\"x\", %d)\nend" % (v, i)
        f.fn(
            "cg_println_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"println!(\\\"{}\\\", %d)\"));\n"
            "    assert!(rust.contains(\"println!(\\\"{} {}\\\", \\\"x\\\", %d)\"));\n"
            "    assert!(!rust.contains(\"print(\"));\n" % (raw(src), v, i),
        )

    # 5. for i in 0..N emission
    for i in range(1, 16):
        n = 10 + i
        src = "fn main() do\n  var s = 0\n  for i in %d do\n    s = s + i\n  end\n  print(s)\nend" % n
        f.fn(
            "cg_for_range_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"for i in 0..%d {\"));\n" % (raw(src), n),
        )

    # 6. repeat -> loop { ... if cond { break; } }
    for i in range(1, 16):
        n = 2 + i
        src = "fn main() do\n  var c = 0\n  repeat\n    c = c + 1\n  until c >= %d\n  print(c)\nend" % n
        f.fn(
            "cg_repeat_loop_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"loop {\"));\n"
            "    assert!(rust.contains(\"break;\"));\n"
            "    assert!(rust.contains(\"if c >= %d {\"));\n" % (raw(src), n),
        )

    # 7. .to_string() for String fields built from literals
    for i in range(1, 16):
        v = i * 8
        src = ("N = struct\n  id: Int = 0\n  label: String = \"x\"\nend\n\n"
               "fn main() do\n  let n = N { id = %d, label = \"tag%d\" }\n  print(n.id)\nend") % (v, i)
        f.fn(
            "cg_to_string_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"label: \\\"tag%d\\\".to_string()\"));\n"
            "    assert!(rust.contains(\"id: %d,\"));\n" % (raw(src), i, v),
        )

    # 8. let mut inference
    for i in range(1, 16):
        v = i
        src = "fn main() do\n  let a = %d\n  let b = a + 1\n  var c = b\n  c = c + 2\n  print(c)\nend" % v
        f.fn(
            "cg_let_mut_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"let a = %d;\"));\n"
            "    assert!(rust.contains(\"let b = a + 1;\"));\n"
            "    assert!(rust.contains(\"let mut c = b;\"));\n" % (raw(src), v),
        )

    # 9. reassigned let becomes let mut
    for i in range(1, 16):
        v = i * 2
        src = "fn main() do\n  let t = %d\n  t = t + 5\n  print(t)\nend" % v
        f.fn(
            "cg_let_reassign_mut_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"let mut t = %d;\"));\n" % (raw(src), v),
        )

    # 10. pipe x |> f becomes f(x)
    for i in range(1, 16):
        v = i
        src = ("fn double(x) do\n  return x * 2\nend\n\n"
               "fn main() do\n  let y = %d |> double\n  print(y)\nend") % v
        f.fn(
            "cg_pipe_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"let y = double(%d);\"));\n"
            "    assert!(!rust.contains(\"|>\"));\n" % (raw(src), v),
        )

    # 11. struct literal emission with int fields
    for i in range(1, 16):
        a = i * 3
        b = i * 5
        src = ("V = struct\n  x: Int = 0\n  y: Int = 0\nend\n\n"
               "fn main() do\n  let v = V { x = %d, y = %d }\n  print(v.x)\nend") % (a, b)
        f.fn(
            "cg_struct_lit_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"V { x: %d, y: %d }\"));\n" % (raw(src), a, b),
        )

    # 12. struct field type mapping
    for i in range(1, 11):
        src = ("T%d = struct\n  a: Int\n  b: Float\n  c: Bool\n  d: String\nend\n\n"
               "fn main() do\n  print(1)\nend") % i
        f.fn(
            "cg_struct_types_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"a: i64,\"));\n"
            "    assert!(rust.contains(\"b: f64,\"));\n"
            "    assert!(rust.contains(\"c: bool,\"));\n"
            "    assert!(rust.contains(\"d: String,\"));\n" % raw(src),
        )

    # 13. fn signature type mapping
    for i in range(1, 11):
        src = "fn calc(a: Int, b: Float) -> Int do\n  return a\nend\n\nfn main() do\n  print(calc(1, 2))\nend"
        f.fn(
            "cg_fn_types_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"fn calc(a: i64, b: f64) -> i64 {\"));\n" % raw(src),
        )

    # 14. FFI: cblock function extraction (extern "C" surface)
    c_bodies = [
        "int native_add(int a, int b) { return a + b; }",
        "long native_sum(long a, long b) { return a + b; }",
        "double native_mul(double a, double b) { return a * b; }",
        "float native_half(float a) { return a / 2; }",
        "bool native_eq(int a, int b) { return a == b; }",
        "void native_noop() { }",
        "short native_short(short s) { return s; }",
        "char native_char(char c) { return c; }",
        "size_t native_len(const char* s) { return 0; }",
        "int64_t native_i64(int64_t v) { return v; }",
        "int32_t native_i32(int32_t v) { return v; }",
        "int8_t native_i8(int8_t v) { return v; }",
        "int16_t native_i16(int16_t v) { return v; }",
        "void* native_alloc(size_t n) { return 0; }",
        "const char* native_name() { return \"x\"; }",
    ]
    ret_map = ["i32", "i64", "f64", "f32", "bool", "()", "i16", "i8", "usize", "i64", "i32", "i8", "i16",
               "*mut std::ffi::c_void", "*const std::ffi::c_char"]
    for i in range(1, 16):
        body = c_bodies[(i - 1) % len(c_bodies)]
        name = body.split("(")[0].split()[-1]
        ret = ret_map[(i - 1) % len(ret_map)]
        f.fn(
            "cg_ffi_extract_%d" % i,
            "    let fns = ffi::extract_c_functions(%s);\n"
            "    assert_eq!(fns.len(), 1);\n"
            "    assert_eq!(fns[0].name, \"%s\");\n"
            "    assert_eq!(fns[0].ret, \"%s\");\n" % (raw(body), name, ret),
        )

    # 15. cpp mangling
    for i in range(1, 11):
        name = "cpp_fn_%d" % i
        f.fn(
            "cg_ffi_mangle_%d" % i,
            "    let mangled = ffi::cpp_mangle(\"%s\", &[]);\n"
            "    assert_eq!(mangled, \"_Z%d%sv\");\n" % (name, len(name), name),
        )

    # 16. full-pipeline compilation: generated Rust compiles cleanly for the function subset
    for i in range(1, 11):
        n = 100 * i
        total = n * (n - 1) // 2
        src = ("fn work(n: Int) -> Int do\n  var t = 0\n  for i in %d do\n    t = t + i\n  end\n  return t\nend\n\n"
               "fn main() do\n  print(work(3))\nend") % n
        f.fn(
            "cg_pipeline_text_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"fn work(n: i64) -> i64 {\"));\n"
            "    assert!(rust.contains(\"for i in 0..%d {\"));\n"
            "    assert!(rust.contains(\"return t;\"));\n"
            "    assert!(rust.contains(\"fn main() {\"));\n"
            "    assert!(rust.contains(\"println!\"));\n" % (raw(src), n),
        )

    # 17. array codegen
    for i in range(1, 11):
        a = i
        b = i + 1
        c = i + 2
        src = "fn main() do\n  var xs = [%d, %d, %d]\n  xs.push(%d)\n  print(xs.len())\n  print(xs[2])\nend" % (a, b, c, i * 10)
        f.fn(
            "cg_array_%d" % i,
            "    let (rust, diags) = compile_to_rust(%s, 0);\n"
            "    assert!(!diags.has_errors());\n"
            "    assert!(rust.contains(\"let mut xs = vec![%d, %d, %d];\"));\n"
            "    assert!(rust.contains(\"xs.push(%d)\"));\n"
            "    assert!(rust.contains(\"xs.len()\"));\n"
            "    assert!(rust.contains(\"xs[2]\"));\n" % (raw(src), a, b, c, i * 10),
        )

    return f


def main():
    files = [gen_structs(), gen_methods(), gen_ranges(), gen_repeat(), gen_ownership(), gen_codegen_new()]
    total_fns = 0
    total_asserts = 0
    for f in files:
        n_fn, n_assert = emit_output(f)
        total_fns += n_fn
        total_asserts += n_assert
    print("-" * 60)
    print("TOTAL: fns=%d assertions=%d" % (total_fns, total_asserts))


if __name__ == "__main__":
    main()
