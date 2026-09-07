#!/usr/bin/env python3
# Owner: PascalElixir / axolrs (GitHub org)
# File: Tree-sitter corpus generator - parses snippets and records their trees as corpus tests.

import subprocess
import os
import re

CORPUS_DIR = "test/corpus"

SPAN_RE = re.compile(r" \[\d+, \d+\] - \[\d+, \d+\]")

SNIPPETS = {
    "basics.txt": [
        ("empty_source", ""),
        ("hello_fn", "fn main()\n    print(\"hello\")\nend\n"),
        ("let_int", "fn main()\n    let x = 42\nend\n"),
        ("var_bool", "fn main()\n    var flag = true\nend\n"),
        ("line_comment", "-- a comment\nfn main() end\n"),
        ("block_comment", "--[[ block comment ]]\nfn main() end\n"),
        ("float_literal", "fn main() let x = 3.14 end\n"),
        ("hex_literal", "fn main() let x = 0xFF end\n"),
        ("underscore_number", "fn main() let x = 1_000_000 end\n"),
        ("string_escape", "fn main() let s = \"a\\nb\\t\\\\\\\"\" end\n"),
        ("interpolated_string", "fn main() let s = \"count: ${x}\" end\n"),
        ("raw_string", "fn main() let s = `raw $string` end\n"),
        ("char_literal", "fn main() let c = 'a' end\n"),
        ("null_literal", "fn main() let x = null end\n"),
        ("nil_literal", "fn main() let x = nil end\n"),
        ("tuple_expr", "fn main() let t = (1, 2, 3) end\n"),
        ("array_expr", "fn main() let a = [1, 2, 3] end\n"),
        ("record_literal", "fn main() let c = { host = \"localhost\", port = 8080 } end\n"),
        ("pipe_forward", "fn main() let y = x |> f end\n"),
    ],
    "functions.txt": [
        ("fn_no_args", "fn f() end\n"),
        ("fn_typed_params", "fn add(a: Int, b: Int) -> Int\n    return a + b\nend\n"),
        ("pub_fn", "pub fn exported() end\n"),
        ("async_fn", "async fn work() end\n"),
        ("extern_fn", "extern fn SDL_Init(flags: U32) -> Int\n"),
        ("generic_fn", "fn identity<T>(x: T) -> T\n    return x\nend\n"),
        ("default_param", "fn greet(name = \"world\") end\n"),
        ("named_call", "fn main() f(x: 1, y: 2) end\n"),
        ("closure_fn_expr", "fn main() let f = fn(x) return x * 2 end end\n"),
        ("bar_closure", "fn main() let f = |x| x + 1 end\n"),
        ("call_chain", "fn main() a(b(c(1))) end\n"),
        ("method_def", "Point.sum = fn(self, x: Int) -> Int\n    return self.x + x\nend\n"),
        ("static_method", "Vec3 = fn new(x: Int, y: Int)\n    return Vec3 { x = x, y = y }\nend\n"),
        ("method_call_dot", "fn main() p.damage(20) end\n"),
        ("method_call_colon", "fn main() p:damage(20) end\n"),
        ("await_expr", "fn main() let r = await fetch() end\n"),
    ],
    "types.txt": [
        ("int_type", "fn f(x: Int) end\n"),
        ("sized_int", "fn f(x: I64) end\n"),
        ("unsigned", "fn f(x: U32) end\n"),
        ("float_type", "fn f(x: Float) end\n"),
        ("f64_type", "fn f(x: F64) end\n"),
        ("bool_type", "fn f(x: Bool) end\n"),
        ("string_type", "fn f(x: String) end\n"),
        ("optional_type", "fn f(x: Int?) end\n"),
        ("array_type", "fn f(x: [Int]) end\n"),
        ("map_type", "fn f(x: Map<String, Int>) end\n"),
        ("set_type", "fn f(x: Set<Int>) end\n"),
        ("tuple_type", "fn f(x: (Int, String)) end\n"),
        ("borrow_type", "fn f(x: borrow String) end\n"),
        ("mut_type", "fn f(x: mut Player) end\n"),
        ("move_type", "fn f(x: move Data) end\n"),
        ("ref_type", "fn f(x: &String) end\n"),
        ("named_generic", "fn f(x: List<Int>) end\n"),
        ("const_decl", "const MAX: Int = 100\n"),
        ("type_alias", "type Point = (Int, Int)\n"),
    ],
    "control_flow.txt": [
        ("if_end", "fn main()\n    if x > 0 then\n        print(1)\n    end\nend\n"),
        ("if_elseif_else", "fn main()\n    if a then\n        print(1)\n    elseif b then\n        print(2)\n    else\n        print(3)\n    end\nend\n"),
        ("while_do", "fn main()\n    while i < 10 do\n        i = i + 1\n    end\nend\n"),
        ("repeat_until", "fn main()\n    repeat\n        print(\"x\")\n    until done\nend\n"),
        ("numeric_for", "fn main()\n    for i in 10 do\n        print(i)\n    end\nend\n"),
        ("range_for", "fn main()\n    for i in 0..10 do\n        print(i)\n    end\nend\n"),
        ("inclusive_range", "fn main()\n    for i in 0..=9 do\n        print(i)\n    end\nend\n"),
        ("loop_break", "fn main()\n    loop\n        break\n    end\nend\n"),
        ("continue_stmt", "fn main()\n    while true do\n        continue\n    end\nend\n"),
        ("match_literals", "fn main()\n    match x\n        1 => print(10)\n        2 => print(20)\n        _ => print(0)\n    end\nend\n"),
        ("match_guard", "fn main()\n    match p\n        q if q.health <= 0 => die()\n        q => q.update(0.016)\n    end\nend\n"),
        ("match_tuple", "fn main()\n    match position\n        (0, 0) => print(\"origin\")\n        other => print(other)\n    end\nend\n"),
        ("match_enum_pattern", "fn main()\n    match state\n        Menu => show_menu()\n        Playing(score) => tick(score)\n        _ => stop()\n    end\nend\n"),
        ("nested_match", "fn main()\n    match x\n        1 => match y\n            2 => print(12)\n            _ => print(10)\n        end\n        _ => print(0)\n    end\nend\n"),
        ("compound_assign", "fn main()\n    x += 1\n    y -= 2\n    z *= 3\nend\n"),
        ("bitwise_assign", "fn main()\n    a &= 1\n    b |= 2\n    c ^= 3\n    d <<= 4\nend\n"),
    ],
    "structs_methods.txt": [
        ("struct_decl", "Point = struct\n    x: Int\n    y: Int\nend\n"),
        ("struct_pub_field", "Player = struct\n    pub health: Int\nend\n"),
        ("struct_field_default", "Config = struct\n    host: String = \"localhost\"\n    port: Int = 8080\nend\n"),
        ("struct_generic", "Box = struct\n    value: T\nend\n"),
        ("enum_decl", "Color = enum\n    Red\n    Green\n    Blue\nend\n"),
        ("enum_tuple_variant", "Shape = enum\n    Circle(Float)\n    Rect(Float, Float)\nend\n"),
        ("enum_struct_variant", "Event = enum\n    Key { code: Int }\n    Move { dx: Int, dy: Int }\nend\n"),
        ("interface_decl", "Drawable = interface\n    draw(self)\n    area(self) -> Float\nend\n"),
        ("impl_decl", "Circle: Drawable\n"),
        ("struct_literal", "fn main()\n    let p = Player { health = 100, name = \"Mohi\" }\nend\n"),
        ("field_access", "fn main() print(player.health) end\n"),
        ("field_assign", "fn main()\n    player.health -= 10\nend\n"),
        ("index_expr", "fn main() print(xs[0]) end\n"),
        ("optional_chain", "fn main() print(user?.name) end\n"),
        ("bang_unwrap", "fn main() print(box!) end\n"),
        ("question_try", "fn main() let v = parse(s)? end\n"),
        ("spawn_expr", "fn main() spawn work() end\n"),
    ],
    "ffi_blocks.txt": [
        ("cblock", "cblock\n    #include <stdio.h>\n    int native_add(int a, int b) {\n        return a + b;\n    }\nend\n"),
        ("cppblock", "cppblock\n    #include <iostream>\n    void hello_cpp() {\n        std::cout << \"hi\" << std::endl;\n    }\nend\n"),
        ("rblock", "rblock\n    #[inline]\n    pub fn fast_math(x: f32) -> f32 {\n        x.sqrt()\n    }\nend\n"),
        ("pyblock", "pyblock\n    def process(path):\n        return baked(path)\nend\n"),
        ("use_decl", "use \"serde\"\n"),
        ("use_path", "use std::collections::HashMap\n"),
    ],
}


def parse_snippet(snippet: str) -> str:
    with open("/tmp/ts_snippet.axol", "w") as f:
        f.write(snippet)
    result = subprocess.run(
        ["tree-sitter", "parse", "/tmp/ts_snippet.axol"],
        capture_output=True,
        text=True,
        timeout=60,
    )
    out = result.stdout
    lines = []
    for line in out.splitlines():
        if line.startswith("Warning") or line.startswith("Please") or line.startswith("configuration"):
            continue
        if "/tmp/ts_snippet.axol" in line and line.strip().startswith("/tmp"):
            continue
        lines.append(SPAN_RE.sub("", line))
    return "\n".join(lines).strip()


def main() -> None:
    os.makedirs(CORPUS_DIR, exist_ok=True)
    total = 0
    for fname, cases in SNIPPETS.items():
        parts = []
        for name, src in cases:
            tree = parse_snippet(src)
            has_error = "ERROR" in tree or "MISSING" in tree
            if has_error:
                print(f"SKIP (parse errors): {fname}::{name}")
                continue
            parts.append(f"==================\n{name}\n==================\n\n{src}\n---\n\n{tree}\n")
            total += 1
        with open(os.path.join(CORPUS_DIR, fname), "w") as f:
            f.write("\n".join(parts))
        print(f"wrote {fname}: {len(parts)} cases")
    print(f"total corpus cases: {total}")


if __name__ == "__main__":
    main()
