// Owner: PascalElixir / axolrs (GitHub org)
// File: Auto-generated tests for struct declarations, literals, field reads, field mutation (single and nested), field defaults, and struct display.

use axolc_core::interpret;
use axolc_core::parse;
use axolc_core::compile_to_rust;

#[test]
fn struct_decl_fields_1() {
    let (module, diags) = parse(r#"S1 = struct
    f1: Int = 1
    f2: Int = 2
    f3: Int = 3
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S1");
            assert_eq!(s.fields.len(), 3);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_2() {
    let (module, diags) = parse(r#"S2 = struct
    f1: Int = 2
    f2: Int = 4
    f3: Int = 6
    f4: Int = 8
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S2");
            assert_eq!(s.fields.len(), 4);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_3() {
    let (module, diags) = parse(r#"S3 = struct
    f1: Int = 3
    f2: Int = 6
    f3: Int = 9
    f4: Int = 12
    f5: Int = 15
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S3");
            assert_eq!(s.fields.len(), 5);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_4() {
    let (module, diags) = parse(r#"S4 = struct
    f1: Int = 4
    f2: Int = 8
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S4");
            assert_eq!(s.fields.len(), 2);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_5() {
    let (module, diags) = parse(r#"S5 = struct
    f1: Int = 5
    f2: Int = 10
    f3: Int = 15
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S5");
            assert_eq!(s.fields.len(), 3);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_6() {
    let (module, diags) = parse(r#"S6 = struct
    f1: Int = 6
    f2: Int = 12
    f3: Int = 18
    f4: Int = 24
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S6");
            assert_eq!(s.fields.len(), 4);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_7() {
    let (module, diags) = parse(r#"S7 = struct
    f1: Int = 7
    f2: Int = 14
    f3: Int = 21
    f4: Int = 28
    f5: Int = 35
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S7");
            assert_eq!(s.fields.len(), 5);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_8() {
    let (module, diags) = parse(r#"S8 = struct
    f1: Int = 8
    f2: Int = 16
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S8");
            assert_eq!(s.fields.len(), 2);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_9() {
    let (module, diags) = parse(r#"S9 = struct
    f1: Int = 9
    f2: Int = 18
    f3: Int = 27
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S9");
            assert_eq!(s.fields.len(), 3);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_10() {
    let (module, diags) = parse(r#"S10 = struct
    f1: Int = 10
    f2: Int = 20
    f3: Int = 30
    f4: Int = 40
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S10");
            assert_eq!(s.fields.len(), 4);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_11() {
    let (module, diags) = parse(r#"S11 = struct
    f1: Int = 11
    f2: Int = 22
    f3: Int = 33
    f4: Int = 44
    f5: Int = 55
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S11");
            assert_eq!(s.fields.len(), 5);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_12() {
    let (module, diags) = parse(r#"S12 = struct
    f1: Int = 12
    f2: Int = 24
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S12");
            assert_eq!(s.fields.len(), 2);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_13() {
    let (module, diags) = parse(r#"S13 = struct
    f1: Int = 13
    f2: Int = 26
    f3: Int = 39
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S13");
            assert_eq!(s.fields.len(), 3);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_14() {
    let (module, diags) = parse(r#"S14 = struct
    f1: Int = 14
    f2: Int = 28
    f3: Int = 42
    f4: Int = 56
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S14");
            assert_eq!(s.fields.len(), 4);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_15() {
    let (module, diags) = parse(r#"S15 = struct
    f1: Int = 15
    f2: Int = 30
    f3: Int = 45
    f4: Int = 60
    f5: Int = 75
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S15");
            assert_eq!(s.fields.len(), 5);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_16() {
    let (module, diags) = parse(r#"S16 = struct
    f1: Int = 16
    f2: Int = 32
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S16");
            assert_eq!(s.fields.len(), 2);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_17() {
    let (module, diags) = parse(r#"S17 = struct
    f1: Int = 17
    f2: Int = 34
    f3: Int = 51
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S17");
            assert_eq!(s.fields.len(), 3);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_18() {
    let (module, diags) = parse(r#"S18 = struct
    f1: Int = 18
    f2: Int = 36
    f3: Int = 54
    f4: Int = 72
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S18");
            assert_eq!(s.fields.len(), 4);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_19() {
    let (module, diags) = parse(r#"S19 = struct
    f1: Int = 19
    f2: Int = 38
    f3: Int = 57
    f4: Int = 76
    f5: Int = 95
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S19");
            assert_eq!(s.fields.len(), 5);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_decl_fields_20() {
    let (module, diags) = parse(r#"S20 = struct
    f1: Int = 20
    f2: Int = 40
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(module.items.len(), 1);
    let mut found = 0;
    for item in &module.items {
        if let axolc_core::ast::Item::Struct(s) = item {
            assert_eq!(s.name.name, "S20");
            assert_eq!(s.fields.len(), 2);
            found += 1;
        }
    }
    assert_eq!(found, 1);
}

#[test]
fn struct_read_xy_1() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 3, y = 5 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
5
8
"#);
}

#[test]
fn struct_read_xy_2() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 6, y = 12 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
12
18
"#);
}

#[test]
fn struct_read_xy_3() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 9, y = 19 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"9
19
28
"#);
}

#[test]
fn struct_read_xy_4() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 12, y = 26 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
26
38
"#);
}

#[test]
fn struct_read_xy_5() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 15, y = 33 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
33
48
"#);
}

#[test]
fn struct_read_xy_6() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 18, y = 40 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
40
58
"#);
}

#[test]
fn struct_read_xy_7() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 21, y = 47 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
47
68
"#);
}

#[test]
fn struct_read_xy_8() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 24, y = 54 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"24
54
78
"#);
}

#[test]
fn struct_read_xy_9() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 27, y = 61 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"27
61
88
"#);
}

#[test]
fn struct_read_xy_10() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 30, y = 68 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
68
98
"#);
}

#[test]
fn struct_read_xy_11() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 33, y = 75 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"33
75
108
"#);
}

#[test]
fn struct_read_xy_12() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 36, y = 82 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"36
82
118
"#);
}

#[test]
fn struct_read_xy_13() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 39, y = 89 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"39
89
128
"#);
}

#[test]
fn struct_read_xy_14() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 42, y = 96 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"42
96
138
"#);
}

#[test]
fn struct_read_xy_15() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 45, y = 103 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"45
103
148
"#);
}

#[test]
fn struct_read_xy_16() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 48, y = 110 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"48
110
158
"#);
}

#[test]
fn struct_read_xy_17() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 51, y = 117 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"51
117
168
"#);
}

#[test]
fn struct_read_xy_18() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 54, y = 124 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"54
124
178
"#);
}

#[test]
fn struct_read_xy_19() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 57, y = 131 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"57
131
188
"#);
}

#[test]
fn struct_read_xy_20() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 60, y = 138 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"60
138
198
"#);
}

#[test]
fn struct_read_xy_21() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 63, y = 145 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"63
145
208
"#);
}

#[test]
fn struct_read_xy_22() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 66, y = 152 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"66
152
218
"#);
}

#[test]
fn struct_read_xy_23() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 69, y = 159 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"69
159
228
"#);
}

#[test]
fn struct_read_xy_24() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 72, y = 166 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"72
166
238
"#);
}

#[test]
fn struct_read_xy_25() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 75, y = 173 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"75
173
248
"#);
}

#[test]
fn struct_read_xy_26() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 78, y = 180 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"78
180
258
"#);
}

#[test]
fn struct_read_xy_27() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 81, y = 187 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"81
187
268
"#);
}

#[test]
fn struct_read_xy_28() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 84, y = 194 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"84
194
278
"#);
}

#[test]
fn struct_read_xy_29() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 87, y = 201 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"87
201
288
"#);
}

#[test]
fn struct_read_xy_30() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 90, y = 208 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"90
208
298
"#);
}

#[test]
fn struct_read_xy_31() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 93, y = 215 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_31: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"93
215
308
"#);
}

#[test]
fn struct_read_xy_32() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 96, y = 222 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_32: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"96
222
318
"#);
}

#[test]
fn struct_read_xy_33() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 99, y = 229 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_33: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"99
229
328
"#);
}

#[test]
fn struct_read_xy_34() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 102, y = 236 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_34: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"102
236
338
"#);
}

#[test]
fn struct_read_xy_35() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 105, y = 243 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_35: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"105
243
348
"#);
}

#[test]
fn struct_read_xy_36() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 108, y = 250 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_36: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"108
250
358
"#);
}

#[test]
fn struct_read_xy_37() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 111, y = 257 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_37: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"111
257
368
"#);
}

#[test]
fn struct_read_xy_38() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 114, y = 264 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_38: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"114
264
378
"#);
}

#[test]
fn struct_read_xy_39() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 117, y = 271 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_39: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"117
271
388
"#);
}

#[test]
fn struct_read_xy_40() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 120, y = 278 }
  print(p.x)
  print(p.y)
  print(p.x + p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_xy_40: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"120
278
398
"#);
}

#[test]
fn struct_ctor_positional_1() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(11, 89)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"11
89
979
"#);
}

#[test]
fn struct_ctor_positional_2() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(12, 88)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
88
1056
"#);
}

#[test]
fn struct_ctor_positional_3() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(13, 87)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"13
87
1131
"#);
}

#[test]
fn struct_ctor_positional_4() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(14, 86)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
86
1204
"#);
}

#[test]
fn struct_ctor_positional_5() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(15, 85)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
85
1275
"#);
}

#[test]
fn struct_ctor_positional_6() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(16, 84)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
84
1344
"#);
}

#[test]
fn struct_ctor_positional_7() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(17, 83)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
83
1411
"#);
}

#[test]
fn struct_ctor_positional_8() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(18, 82)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
82
1476
"#);
}

#[test]
fn struct_ctor_positional_9() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(19, 81)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
81
1539
"#);
}

#[test]
fn struct_ctor_positional_10() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(20, 80)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
80
1600
"#);
}

#[test]
fn struct_ctor_positional_11() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(21, 79)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
79
1659
"#);
}

#[test]
fn struct_ctor_positional_12() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(22, 78)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
78
1716
"#);
}

#[test]
fn struct_ctor_positional_13() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(23, 77)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"23
77
1771
"#);
}

#[test]
fn struct_ctor_positional_14() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(24, 76)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"24
76
1824
"#);
}

#[test]
fn struct_ctor_positional_15() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(25, 75)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"25
75
1875
"#);
}

#[test]
fn struct_ctor_positional_16() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(26, 74)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"26
74
1924
"#);
}

#[test]
fn struct_ctor_positional_17() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(27, 73)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"27
73
1971
"#);
}

#[test]
fn struct_ctor_positional_18() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(28, 72)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"28
72
2016
"#);
}

#[test]
fn struct_ctor_positional_19() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(29, 71)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"29
71
2059
"#);
}

#[test]
fn struct_ctor_positional_20() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(30, 70)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
70
2100
"#);
}

#[test]
fn struct_ctor_positional_21() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(31, 69)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"31
69
2139
"#);
}

#[test]
fn struct_ctor_positional_22() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(32, 68)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"32
68
2176
"#);
}

#[test]
fn struct_ctor_positional_23() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(33, 67)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"33
67
2211
"#);
}

#[test]
fn struct_ctor_positional_24() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(34, 66)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"34
66
2244
"#);
}

#[test]
fn struct_ctor_positional_25() {
    let (out, diags) = interpret(r#"Vec = struct
  x: Int = 0
  y: Int = 0
end

fn main() do
  let v = Vec(35, 65)
  print(v.x)
  print(v.y)
  print(v.x * v.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_ctor_positional_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"35
65
2275
"#);
}

#[test]
fn struct_mut_single_1() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 5 }
  p.x = 3
  print(p.x)
  p.x = p.x + 1
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"3
4
"#);
}

#[test]
fn struct_mut_single_2() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 10 }
  p.x = 5
  print(p.x)
  p.x = p.x + 2
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"5
7
"#);
}

#[test]
fn struct_mut_single_3() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 15 }
  p.x = 7
  print(p.x)
  p.x = p.x + 3
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"7
10
"#);
}

#[test]
fn struct_mut_single_4() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 20 }
  p.x = 9
  print(p.x)
  p.x = p.x + 4
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"9
13
"#);
}

#[test]
fn struct_mut_single_5() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 25 }
  p.x = 11
  print(p.x)
  p.x = p.x + 5
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"11
16
"#);
}

#[test]
fn struct_mut_single_6() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 30 }
  p.x = 13
  print(p.x)
  p.x = p.x + 6
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"13
19
"#);
}

#[test]
fn struct_mut_single_7() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 35 }
  p.x = 15
  print(p.x)
  p.x = p.x + 7
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"15
22
"#);
}

#[test]
fn struct_mut_single_8() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 40 }
  p.x = 17
  print(p.x)
  p.x = p.x + 8
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"17
25
"#);
}

#[test]
fn struct_mut_single_9() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 45 }
  p.x = 19
  print(p.x)
  p.x = p.x + 9
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"19
28
"#);
}

#[test]
fn struct_mut_single_10() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 50 }
  p.x = 21
  print(p.x)
  p.x = p.x + 10
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"21
31
"#);
}

#[test]
fn struct_mut_single_11() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 55 }
  p.x = 23
  print(p.x)
  p.x = p.x + 11
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"23
34
"#);
}

#[test]
fn struct_mut_single_12() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 60 }
  p.x = 25
  print(p.x)
  p.x = p.x + 12
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"25
37
"#);
}

#[test]
fn struct_mut_single_13() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 65 }
  p.x = 27
  print(p.x)
  p.x = p.x + 13
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"27
40
"#);
}

#[test]
fn struct_mut_single_14() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 70 }
  p.x = 29
  print(p.x)
  p.x = p.x + 14
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"29
43
"#);
}

#[test]
fn struct_mut_single_15() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 75 }
  p.x = 31
  print(p.x)
  p.x = p.x + 15
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"31
46
"#);
}

#[test]
fn struct_mut_single_16() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 80 }
  p.x = 33
  print(p.x)
  p.x = p.x + 16
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"33
49
"#);
}

#[test]
fn struct_mut_single_17() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 85 }
  p.x = 35
  print(p.x)
  p.x = p.x + 17
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"35
52
"#);
}

#[test]
fn struct_mut_single_18() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 90 }
  p.x = 37
  print(p.x)
  p.x = p.x + 18
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"37
55
"#);
}

#[test]
fn struct_mut_single_19() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 95 }
  p.x = 39
  print(p.x)
  p.x = p.x + 19
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"39
58
"#);
}

#[test]
fn struct_mut_single_20() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 100 }
  p.x = 41
  print(p.x)
  p.x = p.x + 20
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"41
61
"#);
}

#[test]
fn struct_mut_single_21() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 105 }
  p.x = 43
  print(p.x)
  p.x = p.x + 21
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"43
64
"#);
}

#[test]
fn struct_mut_single_22() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 110 }
  p.x = 45
  print(p.x)
  p.x = p.x + 22
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"45
67
"#);
}

#[test]
fn struct_mut_single_23() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 115 }
  p.x = 47
  print(p.x)
  p.x = p.x + 23
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"47
70
"#);
}

#[test]
fn struct_mut_single_24() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 120 }
  p.x = 49
  print(p.x)
  p.x = p.x + 24
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"49
73
"#);
}

#[test]
fn struct_mut_single_25() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 125 }
  p.x = 51
  print(p.x)
  p.x = p.x + 25
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"51
76
"#);
}

#[test]
fn struct_mut_single_26() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 130 }
  p.x = 53
  print(p.x)
  p.x = p.x + 26
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"53
79
"#);
}

#[test]
fn struct_mut_single_27() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 135 }
  p.x = 55
  print(p.x)
  p.x = p.x + 27
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"55
82
"#);
}

#[test]
fn struct_mut_single_28() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 140 }
  p.x = 57
  print(p.x)
  p.x = p.x + 28
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"57
85
"#);
}

#[test]
fn struct_mut_single_29() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 145 }
  p.x = 59
  print(p.x)
  p.x = p.x + 29
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"59
88
"#);
}

#[test]
fn struct_mut_single_30() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 150 }
  p.x = 61
  print(p.x)
  p.x = p.x + 30
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_single_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"61
91
"#);
}

#[test]
fn struct_mut_compound_1() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 4, y = 5 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
3
"#);
}

#[test]
fn struct_mut_compound_2() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 8, y = 9 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
7
"#);
}

#[test]
fn struct_mut_compound_3() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 12, y = 13 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
11
"#);
}

#[test]
fn struct_mut_compound_4() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 16, y = 17 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"38
15
"#);
}

#[test]
fn struct_mut_compound_5() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 20, y = 21 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"46
19
"#);
}

#[test]
fn struct_mut_compound_6() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 24, y = 25 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"54
23
"#);
}

#[test]
fn struct_mut_compound_7() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 28, y = 29 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"62
27
"#);
}

#[test]
fn struct_mut_compound_8() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 32, y = 33 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"70
31
"#);
}

#[test]
fn struct_mut_compound_9() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 36, y = 37 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"78
35
"#);
}

#[test]
fn struct_mut_compound_10() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 40, y = 41 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"86
39
"#);
}

#[test]
fn struct_mut_compound_11() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 44, y = 45 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"94
43
"#);
}

#[test]
fn struct_mut_compound_12() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 48, y = 49 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"102
47
"#);
}

#[test]
fn struct_mut_compound_13() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 52, y = 53 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"110
51
"#);
}

#[test]
fn struct_mut_compound_14() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 56, y = 57 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"118
55
"#);
}

#[test]
fn struct_mut_compound_15() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 60, y = 61 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"126
59
"#);
}

#[test]
fn struct_mut_compound_16() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 64, y = 65 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"134
63
"#);
}

#[test]
fn struct_mut_compound_17() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 68, y = 69 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"142
67
"#);
}

#[test]
fn struct_mut_compound_18() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 72, y = 73 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"150
71
"#);
}

#[test]
fn struct_mut_compound_19() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 76, y = 77 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"158
75
"#);
}

#[test]
fn struct_mut_compound_20() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 80, y = 81 }
  p.x += 3
  p.y -= 2
  p.x *= 2
  print(p.x)
  print(p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_compound_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"166
79
"#);
}

#[test]
fn struct_mut_nested_1() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 2, y = 3 }, mass = 1 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 41
  print(p.pos.x)
  p.pos.y += 1
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"2
3
41
4
1
"#);
}

#[test]
fn struct_mut_nested_2() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 4, y = 6 }, mass = 2 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 42
  print(p.pos.x)
  p.pos.y += 2
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
6
42
8
2
"#);
}

#[test]
fn struct_mut_nested_3() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 6, y = 9 }, mass = 3 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 43
  print(p.pos.x)
  p.pos.y += 3
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"6
9
43
12
3
"#);
}

#[test]
fn struct_mut_nested_4() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 8, y = 12 }, mass = 4 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 44
  print(p.pos.x)
  p.pos.y += 4
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
12
44
16
4
"#);
}

#[test]
fn struct_mut_nested_5() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 10, y = 15 }, mass = 5 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 45
  print(p.pos.x)
  p.pos.y += 5
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"10
15
45
20
5
"#);
}

#[test]
fn struct_mut_nested_6() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 12, y = 18 }, mass = 6 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 46
  print(p.pos.x)
  p.pos.y += 6
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"12
18
46
24
6
"#);
}

#[test]
fn struct_mut_nested_7() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 14, y = 21 }, mass = 7 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 47
  print(p.pos.x)
  p.pos.y += 7
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
21
47
28
7
"#);
}

#[test]
fn struct_mut_nested_8() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 16, y = 24 }, mass = 8 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 48
  print(p.pos.x)
  p.pos.y += 8
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"16
24
48
32
8
"#);
}

#[test]
fn struct_mut_nested_9() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 18, y = 27 }, mass = 9 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 49
  print(p.pos.x)
  p.pos.y += 9
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"18
27
49
36
9
"#);
}

#[test]
fn struct_mut_nested_10() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 20, y = 30 }, mass = 10 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 50
  print(p.pos.x)
  p.pos.y += 10
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"20
30
50
40
10
"#);
}

#[test]
fn struct_mut_nested_11() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 22, y = 33 }, mass = 11 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 51
  print(p.pos.x)
  p.pos.y += 11
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
33
51
44
11
"#);
}

#[test]
fn struct_mut_nested_12() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 24, y = 36 }, mass = 12 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 52
  print(p.pos.x)
  p.pos.y += 12
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"24
36
52
48
12
"#);
}

#[test]
fn struct_mut_nested_13() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 26, y = 39 }, mass = 13 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 53
  print(p.pos.x)
  p.pos.y += 13
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"26
39
53
52
13
"#);
}

#[test]
fn struct_mut_nested_14() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 28, y = 42 }, mass = 14 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 54
  print(p.pos.x)
  p.pos.y += 14
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"28
42
54
56
14
"#);
}

#[test]
fn struct_mut_nested_15() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 30, y = 45 }, mass = 15 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 55
  print(p.pos.x)
  p.pos.y += 15
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"30
45
55
60
15
"#);
}

#[test]
fn struct_mut_nested_16() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 32, y = 48 }, mass = 16 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 56
  print(p.pos.x)
  p.pos.y += 16
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"32
48
56
64
16
"#);
}

#[test]
fn struct_mut_nested_17() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 34, y = 51 }, mass = 17 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 57
  print(p.pos.x)
  p.pos.y += 17
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"34
51
57
68
17
"#);
}

#[test]
fn struct_mut_nested_18() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 36, y = 54 }, mass = 18 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 58
  print(p.pos.x)
  p.pos.y += 18
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"36
54
58
72
18
"#);
}

#[test]
fn struct_mut_nested_19() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 38, y = 57 }, mass = 19 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 59
  print(p.pos.x)
  p.pos.y += 19
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"38
57
59
76
19
"#);
}

#[test]
fn struct_mut_nested_20() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 40, y = 60 }, mass = 20 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 60
  print(p.pos.x)
  p.pos.y += 20
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"40
60
60
80
20
"#);
}

#[test]
fn struct_mut_nested_21() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 42, y = 63 }, mass = 21 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 61
  print(p.pos.x)
  p.pos.y += 21
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_21: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"42
63
61
84
21
"#);
}

#[test]
fn struct_mut_nested_22() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 44, y = 66 }, mass = 22 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 62
  print(p.pos.x)
  p.pos.y += 22
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_22: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"44
66
62
88
22
"#);
}

#[test]
fn struct_mut_nested_23() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 46, y = 69 }, mass = 23 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 63
  print(p.pos.x)
  p.pos.y += 23
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_23: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"46
69
63
92
23
"#);
}

#[test]
fn struct_mut_nested_24() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 48, y = 72 }, mass = 24 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 64
  print(p.pos.x)
  p.pos.y += 24
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_24: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"48
72
64
96
24
"#);
}

#[test]
fn struct_mut_nested_25() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 50, y = 75 }, mass = 25 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 65
  print(p.pos.x)
  p.pos.y += 25
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_25: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"50
75
65
100
25
"#);
}

#[test]
fn struct_mut_nested_26() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 52, y = 78 }, mass = 26 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 66
  print(p.pos.x)
  p.pos.y += 26
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_26: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"52
78
66
104
26
"#);
}

#[test]
fn struct_mut_nested_27() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 54, y = 81 }, mass = 27 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 67
  print(p.pos.x)
  p.pos.y += 27
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_27: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"54
81
67
108
27
"#);
}

#[test]
fn struct_mut_nested_28() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 56, y = 84 }, mass = 28 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 68
  print(p.pos.x)
  p.pos.y += 28
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_28: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"56
84
68
112
28
"#);
}

#[test]
fn struct_mut_nested_29() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 58, y = 87 }, mass = 29 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 69
  print(p.pos.x)
  p.pos.y += 29
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_29: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"58
87
69
116
29
"#);
}

#[test]
fn struct_mut_nested_30() {
    let (out, diags) = interpret(r#"Pos = struct
  x: Int = 0
  y: Int = 0
end

Particle = struct
  pos: Pos = Pos { x = 0, y = 0 }
  mass: Int = 1
end

fn main() do
  let p = Particle { pos = Pos { x = 60, y = 90 }, mass = 30 }
  print(p.pos.x)
  print(p.pos.y)
  p.pos.x = 70
  print(p.pos.x)
  p.pos.y += 30
  print(p.pos.y)
  print(p.mass)
end"#, 0);
    assert!(!diags.has_errors(), "struct_mut_nested_30: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"60
90
70
120
30
"#);
}

#[test]
fn struct_display_single_1() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 11 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 11}
"#);
}

#[test]
fn struct_display_single_2() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 22 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 22}
"#);
}

#[test]
fn struct_display_single_3() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 33 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 33}
"#);
}

#[test]
fn struct_display_single_4() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 44 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 44}
"#);
}

#[test]
fn struct_display_single_5() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 55 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 55}
"#);
}

#[test]
fn struct_display_single_6() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 66 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 66}
"#);
}

#[test]
fn struct_display_single_7() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 77 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 77}
"#);
}

#[test]
fn struct_display_single_8() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 88 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 88}
"#);
}

#[test]
fn struct_display_single_9() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 99 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 99}
"#);
}

#[test]
fn struct_display_single_10() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 110 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 110}
"#);
}

#[test]
fn struct_display_single_11() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 121 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 121}
"#);
}

#[test]
fn struct_display_single_12() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 132 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 132}
"#);
}

#[test]
fn struct_display_single_13() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 143 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 143}
"#);
}

#[test]
fn struct_display_single_14() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 154 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 154}
"#);
}

#[test]
fn struct_display_single_15() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 165 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 165}
"#);
}

#[test]
fn struct_display_single_16() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 176 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 176}
"#);
}

#[test]
fn struct_display_single_17() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 187 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 187}
"#);
}

#[test]
fn struct_display_single_18() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 198 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 198}
"#);
}

#[test]
fn struct_display_single_19() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 209 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 209}
"#);
}

#[test]
fn struct_display_single_20() {
    let (out, diags) = interpret(r#"fn main() do
  let s = Solo { v = 220 }
  print(s)
end"#, 0);
    assert!(!diags.has_errors(), "struct_display_single_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"Solo {v = 220}
"#);
}

#[test]
fn struct_display_multi_1() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 101, y = 3 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 101"));
    assert!(out.contains("y = 3"));
}

#[test]
fn struct_display_multi_2() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 102, y = 5 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 102"));
    assert!(out.contains("y = 5"));
}

#[test]
fn struct_display_multi_3() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 103, y = 7 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 103"));
    assert!(out.contains("y = 7"));
}

#[test]
fn struct_display_multi_4() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 104, y = 9 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 104"));
    assert!(out.contains("y = 9"));
}

#[test]
fn struct_display_multi_5() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 105, y = 11 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 105"));
    assert!(out.contains("y = 11"));
}

#[test]
fn struct_display_multi_6() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 106, y = 13 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 106"));
    assert!(out.contains("y = 13"));
}

#[test]
fn struct_display_multi_7() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 107, y = 15 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 107"));
    assert!(out.contains("y = 15"));
}

#[test]
fn struct_display_multi_8() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 108, y = 17 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 108"));
    assert!(out.contains("y = 17"));
}

#[test]
fn struct_display_multi_9() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 109, y = 19 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 109"));
    assert!(out.contains("y = 19"));
}

#[test]
fn struct_display_multi_10() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 110, y = 21 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 110"));
    assert!(out.contains("y = 21"));
}

#[test]
fn struct_display_multi_11() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 111, y = 23 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 111"));
    assert!(out.contains("y = 23"));
}

#[test]
fn struct_display_multi_12() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 112, y = 25 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 112"));
    assert!(out.contains("y = 25"));
}

#[test]
fn struct_display_multi_13() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 113, y = 27 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 113"));
    assert!(out.contains("y = 27"));
}

#[test]
fn struct_display_multi_14() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 114, y = 29 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 114"));
    assert!(out.contains("y = 29"));
}

#[test]
fn struct_display_multi_15() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 115, y = 31 }
  print(p)
end"#, 0);
    assert!(!diags.has_errors());
    assert_eq!(out.lines().count(), 1);
    assert!(out.contains("P {"));
    assert!(out.contains("x = 115"));
    assert!(out.contains("y = 31"));
}

#[test]
fn struct_default_parse_1() {
    let (module, diags) = parse(r#"D1 = struct
  a: Int = 9
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D1" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), true);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_2() {
    let (module, diags) = parse(r#"D2 = struct
  a: Int
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D2" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), false);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_3() {
    let (module, diags) = parse(r#"D3 = struct
  a: Int = 27
  b: String = "s"
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D3" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), true);
    assert_eq!(s.fields[1].default.is_some(), true);
}

#[test]
fn struct_default_parse_4() {
    let (module, diags) = parse(r#"D4 = struct
  a: Int
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D4" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), false);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_5() {
    let (module, diags) = parse(r#"D5 = struct
  a: Int = 45
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D5" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), true);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_6() {
    let (module, diags) = parse(r#"D6 = struct
  a: Int
  b: String = "s"
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D6" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), false);
    assert_eq!(s.fields[1].default.is_some(), true);
}

#[test]
fn struct_default_parse_7() {
    let (module, diags) = parse(r#"D7 = struct
  a: Int = 63
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D7" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), true);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_8() {
    let (module, diags) = parse(r#"D8 = struct
  a: Int
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D8" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), false);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_9() {
    let (module, diags) = parse(r#"D9 = struct
  a: Int = 81
  b: String = "s"
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D9" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), true);
    assert_eq!(s.fields[1].default.is_some(), true);
}

#[test]
fn struct_default_parse_10() {
    let (module, diags) = parse(r#"D10 = struct
  a: Int
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D10" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), false);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_11() {
    let (module, diags) = parse(r#"D11 = struct
  a: Int = 99
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D11" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), true);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_12() {
    let (module, diags) = parse(r#"D12 = struct
  a: Int
  b: String = "s"
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D12" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), false);
    assert_eq!(s.fields[1].default.is_some(), true);
}

#[test]
fn struct_default_parse_13() {
    let (module, diags) = parse(r#"D13 = struct
  a: Int = 117
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D13" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), true);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_14() {
    let (module, diags) = parse(r#"D14 = struct
  a: Int
  b: String
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D14" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), false);
    assert_eq!(s.fields[1].default.is_some(), false);
}

#[test]
fn struct_default_parse_15() {
    let (module, diags) = parse(r#"D15 = struct
  a: Int = 135
  b: String = "s"
end

fn main() do
  print(1)
end"#, 0);
    assert!(!diags.has_errors());
    let s = module.items.iter().find_map(|it| match it {
        axolc_core::ast::Item::Struct(s) if s.name.name == "D15" => Some(s),
        _ => None,
    });
    let s = s.unwrap();
    assert_eq!(s.fields.len(), 2);
    assert_eq!(s.fields[0].default.is_some(), true);
    assert_eq!(s.fields[1].default.is_some(), true);
}

#[test]
fn struct_codegen_1() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 6, name = "ax1" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 6, name: \"ax1\".to_string() }"));
}

#[test]
fn struct_codegen_2() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 12, name = "ax2" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 12, name: \"ax2\".to_string() }"));
}

#[test]
fn struct_codegen_3() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 18, name = "ax3" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 18, name: \"ax3\".to_string() }"));
}

#[test]
fn struct_codegen_4() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 24, name = "ax4" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 24, name: \"ax4\".to_string() }"));
}

#[test]
fn struct_codegen_5() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 30, name = "ax5" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 30, name: \"ax5\".to_string() }"));
}

#[test]
fn struct_codegen_6() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 36, name = "ax6" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 36, name: \"ax6\".to_string() }"));
}

#[test]
fn struct_codegen_7() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 42, name = "ax7" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 42, name: \"ax7\".to_string() }"));
}

#[test]
fn struct_codegen_8() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 48, name = "ax8" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 48, name: \"ax8\".to_string() }"));
}

#[test]
fn struct_codegen_9() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 54, name = "ax9" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 54, name: \"ax9\".to_string() }"));
}

#[test]
fn struct_codegen_10() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 60, name = "ax10" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 60, name: \"ax10\".to_string() }"));
}

#[test]
fn struct_codegen_11() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 66, name = "ax11" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 66, name: \"ax11\".to_string() }"));
}

#[test]
fn struct_codegen_12() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 72, name = "ax12" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 72, name: \"ax12\".to_string() }"));
}

#[test]
fn struct_codegen_13() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 78, name = "ax13" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 78, name: \"ax13\".to_string() }"));
}

#[test]
fn struct_codegen_14() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 84, name = "ax14" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 84, name: \"ax14\".to_string() }"));
}

#[test]
fn struct_codegen_15() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 90, name = "ax15" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 90, name: \"ax15\".to_string() }"));
}

#[test]
fn struct_codegen_16() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 96, name = "ax16" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 96, name: \"ax16\".to_string() }"));
}

#[test]
fn struct_codegen_17() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 102, name = "ax17" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 102, name: \"ax17\".to_string() }"));
}

#[test]
fn struct_codegen_18() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 108, name = "ax18" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 108, name: \"ax18\".to_string() }"));
}

#[test]
fn struct_codegen_19() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 114, name = "ax19" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 114, name: \"ax19\".to_string() }"));
}

#[test]
fn struct_codegen_20() {
    let (rust, diags) = compile_to_rust(r#"P = struct
  x: Int = 0
  name: String = "n"
end

fn main() do
  let p = P { x = 120, name = "ax20" }
  print(p.x)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct P {"));
    assert!(rust.contains("x: i64,"));
    assert!(rust.contains("name: String,"));
    assert!(rust.contains("P { x: 120, name: \"ax20\".to_string() }"));
}

#[test]
fn struct_codegen_empty_1() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 1
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_codegen_empty_2() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 2
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_codegen_empty_3() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 3
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_codegen_empty_4() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 4
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_codegen_empty_5() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 5
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_codegen_empty_6() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 6
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_codegen_empty_7() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 7
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_codegen_empty_8() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 8
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_codegen_empty_9() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 9
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_codegen_empty_10() {
    let (rust, diags) = compile_to_rust(r#"E = struct
  v: Int = 10
end

fn main() do
  let e = E { }
  print(2)
end"#, 0);
    assert!(!diags.has_errors());
    assert!(rust.contains("struct E {"));
    assert!(rust.contains("E::default()"));
}

#[test]
fn struct_read_expr_1() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 3 + 1, y = 3 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"4
6
-2
"#);
}

#[test]
fn struct_read_expr_2() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 4 + 4, y = 4 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"8
8
0
"#);
}

#[test]
fn struct_read_expr_3() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 5 + 9, y = 5 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"14
10
4
"#);
}

#[test]
fn struct_read_expr_4() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 6 + 16, y = 6 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"22
12
10
"#);
}

#[test]
fn struct_read_expr_5() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 7 + 25, y = 7 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"32
14
18
"#);
}

#[test]
fn struct_read_expr_6() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 8 + 36, y = 8 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"44
16
28
"#);
}

#[test]
fn struct_read_expr_7() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 9 + 49, y = 9 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"58
18
40
"#);
}

#[test]
fn struct_read_expr_8() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 10 + 64, y = 10 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"74
20
54
"#);
}

#[test]
fn struct_read_expr_9() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 11 + 81, y = 11 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"92
22
70
"#);
}

#[test]
fn struct_read_expr_10() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 12 + 100, y = 12 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"112
24
88
"#);
}

#[test]
fn struct_read_expr_11() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 13 + 121, y = 13 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"134
26
108
"#);
}

#[test]
fn struct_read_expr_12() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 14 + 144, y = 14 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"158
28
130
"#);
}

#[test]
fn struct_read_expr_13() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 15 + 169, y = 15 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"184
30
154
"#);
}

#[test]
fn struct_read_expr_14() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 16 + 196, y = 16 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"212
32
180
"#);
}

#[test]
fn struct_read_expr_15() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 17 + 225, y = 17 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"242
34
208
"#);
}

#[test]
fn struct_read_expr_16() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 18 + 256, y = 18 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_16: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"274
36
238
"#);
}

#[test]
fn struct_read_expr_17() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 19 + 289, y = 19 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_17: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"308
38
270
"#);
}

#[test]
fn struct_read_expr_18() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 20 + 324, y = 20 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_18: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"344
40
304
"#);
}

#[test]
fn struct_read_expr_19() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 21 + 361, y = 21 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_19: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"382
42
340
"#);
}

#[test]
fn struct_read_expr_20() {
    let (out, diags) = interpret(r#"fn main() do
  let p = P { x = 22 + 400, y = 22 * 2 }
  print(p.x)
  print(p.y)
  print(p.x - p.y)
end"#, 0);
    assert!(!diags.has_errors(), "struct_read_expr_20: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"422
44
378
"#);
}

#[test]
fn struct_field_eq_1() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 3 }
  let b = P { x = 3 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_1: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_2() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 6 }
  let b = P { x = 6 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_2: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_3() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 9 }
  let b = P { x = 9 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_3: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_4() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 12 }
  let b = P { x = 12 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_4: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_5() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 15 }
  let b = P { x = 15 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_5: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_6() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 18 }
  let b = P { x = 18 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_6: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_7() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 21 }
  let b = P { x = 21 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_7: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_8() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 24 }
  let b = P { x = 24 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_8: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_9() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 27 }
  let b = P { x = 27 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_9: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_10() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 30 }
  let b = P { x = 30 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_10: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_11() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 33 }
  let b = P { x = 33 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_11: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_12() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 36 }
  let b = P { x = 36 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_12: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_13() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 39 }
  let b = P { x = 39 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_13: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_14() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 42 }
  let b = P { x = 42 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_14: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}

#[test]
fn struct_field_eq_15() {
    let (out, diags) = interpret(r#"fn main() do
  let a = P { x = 45 }
  let b = P { x = 45 }
  if a.x == b.x then print("eq") else print("ne") end
end"#, 0);
    assert!(!diags.has_errors(), "struct_field_eq_15: expected no errors, got: {:?}", diags.items);
    assert_eq!(out, r#"eq
"#);
}
