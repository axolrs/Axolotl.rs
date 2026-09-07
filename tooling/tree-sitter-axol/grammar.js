/**
 * @file Tree-sitter grammar for the Axolotl programming language
 * @author PascalElixir <pascalelixir@users.noreply.github.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

/**
 * Binary operator precedence, lowest to highest.
 * Statements and prefix/postfix expressions live above the binary ladder.
 */
const PREC = {
  ASSIGNMENT: 1,
  PIPE: 2,
  OR: 3,
  COALESCE: 4,
  AND: 5,
  COMPARE: 6,
  RANGE: 7,
  BITWISE_OR: 8,
  BITWISE_XOR: 9,
  BITWISE_AND: 10,
  SHIFT: 11,
  ADD: 12,
  MULTIPLY: 13,
  CAST: 14,
  UNARY: 15,
  POSTFIX: 16,
};

/** One or more `rule`, separated by commas. */
function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)));
}

export default grammar({
  name: 'axolotlgrammer',

  word: $ => $.identifier,

  extras: $ => [
    /[\s\uFEFF\u2060\u200B]/,
    $.comment,
    $.block_comment,
    $.doc_comment,
  ],

  conflicts: $ => [
    // `return fn(...) ... end` (returning a closure) vs. a value-less
    // `return` followed by a `fn name(...) ... end` declaration.
    [$.return_statement],
    // `x as List<Int>` (cast to a generic type) vs. `(x as Foo) < y`
    // (a comparison right after a cast). GLR forks and the `<` either
    // opens generic arguments or continues a comparison expression.
    [$._type, $.generic_type],
    // At the boundary between a `match` scrutinee and the first arm,
    // `(` can open a call (scrutinee continues) or a tuple pattern (the
    // arm begins). GLR keeps both forks until `|` / `=>` disambiguate.
    [$.tuple_pattern, $.arguments],
    // Literal atoms (numbers, strings, `null`, identifiers) are both
    // expressions and patterns; at match-arm boundaries GLR keeps both.
    [$._pattern_atom, $._expression],
    [$.variant_pattern, $._expression],
    [$.struct_pattern, $._expression],
    [$.record_pattern, $.record_expression],
    [$.array_pattern, $.array_expression],
  ],

  rules: {
    // ==================================================================
    // File structure
    // ==================================================================

    source_file: $ => repeat(choice($._item, $._statement)),

    _item: $ => choice(
      $.struct_definition,
      $.enum_definition,
      $.interface_definition,
      $.const_assignment,
      $.type_alias,
      $.static_method_definition,
      $.method_definition,
      $.colon_method_definition,
      $.impl_declaration,
      $.cblock,
      $.cppblock,
      $.rblock,
      $.pyblock,
    ),

    // ==================================================================
    // Items
    // ==================================================================

    function_declaration: $ => choice(
      seq(
        repeat(choice('pub', 'async')),
        'fn',
        field('name', $.identifier),
        optional(field('type_parameters', $.type_parameters)),
        field('parameters', $.parameters),
        optional($.return_type),
        optional(field('body', $.block)),
        'end',
      ),
      // `extern fn` declares a foreign function; it has no body.
      seq(
        optional('pub'),
        'extern',
        optional('async'),
        'fn',
        field('name', $.identifier),
        optional(field('type_parameters', $.type_parameters)),
        field('parameters', $.parameters),
        optional($.return_type),
      ),
    ),

    type_parameters: $ => seq(
      '<',
      commaSep1(choice(
        $.identifier,
        seq($.identifier, ':', $._type),
      )),
      optional(','),
      '>',
    ),

    parameters: $ => seq(
      '(',
      optional(seq(
        commaSep1($._parameter),
        optional(','),
      )),
      ')',
    ),

    _parameter: $ => choice(
      $.self_parameter,
      seq(
        field('name', choice(
          $.identifier,
          $.wildcard_pattern,
          $.tuple_pattern,
          $.record_pattern,
        )),
        optional(seq(':', field('type', $._type))),
        optional(seq('=', field('default', $._expression))),
      ),
    ),

    self_parameter: $ => seq('self', optional(seq(':', field('type', $._type)))),

    return_type: $ => prec.right(seq('->', choice(
      $._type,
      seq($._type, ',', commaSep1($._type)),
      seq($._type, '!', $._type),
    ))),

    struct_definition: $ => seq(
      field('name', $.identifier),
      '=',
      'struct',
      optional(field('type_parameters', $.type_parameters)),
      repeat(choice($.field_definition, ',')),
      'end',
    ),

    field_definition: $ => seq(
      optional('pub'),
      field('name', $.identifier),
      ':',
      field('type', $._type),
      optional(seq('=', field('default', $._expression))),
    ),

    enum_definition: $ => seq(
      field('name', $.identifier),
      '=',
      'enum',
      optional(field('type_parameters', $.type_parameters)),
      repeat(choice($.enum_variant, ',')),
      'end',
    ),

    enum_variant: $ => seq(
      field('name', $.identifier),
      optional(choice(
        seq(
          '(',
          optional(seq(commaSep1($._enum_variant_parameter), optional(','))),
          ')',
        ),
        seq('{', repeat(choice($.field_definition, ',')), '}'),
      )),
    ),

    _enum_variant_parameter: $ => choice(
      $._type,
      seq(field('name', $.identifier), ':', field('type', $._type)),
    ),

    interface_definition: $ => seq(
      field('name', $.identifier),
      '=',
      'interface',
      optional(field('type_parameters', $.type_parameters)),
      repeat(choice($.interface_method, ',')),
      'end',
    ),

    interface_method: $ => seq(
      optional('pub'),
      field('name', $.identifier),
      field('parameters', $.parameters),
      optional($.return_type),
      optional(choice(
        seq('=', field('default', $._expression)),
        seq('do', optional(field('body', $.block)), 'end'),
      )),
    ),

    const_declaration: $ => seq(
      optional('pub'),
      'const',
      field('name', $.identifier),
      optional(seq(':', field('type', $._type))),
      '=',
      field('value', $._expression),
    ),

    // `Name = const value` shorthand.
    const_assignment: $ => seq(
      field('name', $.identifier),
      '=',
      'const',
      field('value', $._expression),
    ),

    type_alias: $ => seq(
      optional('pub'),
      'type',
      field('name', $.identifier),
      optional(field('type_parameters', $.type_parameters)),
      '=',
      field('value', choice($._type, $.record_type)),
    ),

    record_type: $ => seq(
      '{',
      repeat(choice($.type_field, ',')),
      '}',
    ),

    type_field: $ => seq(
      field('name', $.identifier),
      ':',
      field('type', $._type),
    ),

    // `Type = fn method_name(params) ... end`
    static_method_definition: $ => prec.right(2, seq(
      field('type', $.identifier),
      '=',
      'fn',
      field('name', $.identifier),
      optional(field('type_parameters', $.type_parameters)),
      field('parameters', $.parameters),
      optional($.return_type),
      optional(field('body', $.block)),
      'end',
    )),

    // `Type.method = fn(self, x: Int) ... end`
    method_definition: $ => prec.right(2, seq(
      optional('pub'),
      field('type', $.identifier),
      '.',
      field('name', $.identifier),
      '=',
      'fn',
      optional(field('type_parameters', $.type_parameters)),
      field('parameters', $.parameters),
      optional($.return_type),
      optional(field('body', $.block)),
      'end',
    )),

    // `Type:method = fn(self, ...) ... end` (implicit-self convention)
    colon_method_definition: $ => prec.right(2, seq(
      optional('pub'),
      field('type', $.identifier),
      ':',
      field('name', $.identifier),
      '=',
      'fn',
      optional(field('type_parameters', $.type_parameters)),
      field('parameters', $.parameters),
      optional($.return_type),
      optional(field('body', $.block)),
      'end',
    )),

    // `Type: Interface` (with optional leading `impl` keyword)
    impl_declaration: $ => prec(1, seq(
      optional('impl'),
      field('type', $.type_identifier),
      ':',
      field('interface', $._type),
    )),

    use_declaration: $ => seq(
      optional('pub'),
      'use',
      choice($.string, $.scoped_identifier),
    ),

    // ==================================================================
    // Foreign code blocks (cblock / cppblock / rblock / pyblock)
    //
    // No external scanner is allowed, so the raw body is a sequence of
    // whole-line tokens and the block closes on a line whose only content
    // is `end`. The closer token includes its trailing newline so it is
    // always the longest match on such a line.
    // ==================================================================

    cblock: $ => seq('cblock', optional($._foreign_code), $._foreign_end),
    cppblock: $ => seq('cppblock', optional($._foreign_code), $._foreign_end),
    rblock: $ => seq('rblock', optional($._foreign_code), $._foreign_end),
    pyblock: $ => seq('pyblock', optional($._foreign_code), $._foreign_end),

    _foreign_code: $ => repeat1($._foreign_line),
    _foreign_line: $ => token(prec(2, /[^\n]+/)),
    _foreign_end: $ => token(prec(3, /[ \t]*end[ \t]*\r?\n/)),

    // ==================================================================
    // Statements
    // ==================================================================

    _statement: $ => choice(
      $.let_statement,
      $.var_statement,
      $.assignment_statement,
      $.expression_statement,
      $.while_statement,
      $.repeat_statement,
      $.for_statement,
      $.loop_statement,
      $.break_statement,
      $.continue_statement,
      $.return_statement,
      $.do_statement,
      $.unsafe_statement,
      $.function_declaration,
      $.const_declaration,
      $.use_declaration,
    ),

    // `prec.right`: when an `else` follows a `let` inside an `if` body, it
    // binds to the `let ... else` (let-else) rather than the `if ... else`.
    let_statement: $ => prec.right(seq(
      'let',
      field('pattern', $._pattern),
      optional(seq(':', field('type', $._type))),
      '=',
      field('value', $._expression),
      optional(seq('else', field('alternative', $._statement))),
    )),

    var_statement: $ => seq(
      'var',
      field('pattern', $._pattern),
      optional(seq(':', field('type', $._type))),
      '=',
      field('value', $._expression),
    ),

    assignment_statement: $ => prec.right(PREC.ASSIGNMENT, seq(
      field('left', commaSep1($._expression)),
      field('operator', $._assignment_operator),
      field('right', commaSep1($._expression)),
    )),

    _assignment_operator: $ => choice(
      '=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '<<=', '>>=',
    ),

    expression_statement: $ => $._expression,

    if_expression: $ => prec.right(seq(
      'if',
      field('condition', $._expression),
      'then',
      optional(field('consequence', $.block)),
      repeat($.elseif_clause),
      optional($.else_clause),
      'end',
    )),

    elseif_clause: $ => seq(
      'elseif',
      field('condition', $._expression),
      'then',
      optional(field('consequence', $.block)),
    ),

    else_clause: $ => seq(
      'else',
      optional(field('alternative', $.block)),
    ),

    while_statement: $ => seq(
      'while',
      field('condition', $._expression),
      'do',
      optional(field('body', $.block)),
      'end',
    ),

    repeat_statement: $ => seq(
      'repeat',
      optional(field('body', $.block)),
      'until',
      field('condition', $._expression),
    ),

    for_statement: $ => seq(
      'for',
      choice(
        seq(
          field('bindings', commaSep1($._for_binding)),
          'in',
          field('iterator', $._expression),
          'do',
          optional(field('body', $.block)),
          'end',
        ),
        seq(
          field('variable', $.identifier),
          '=',
          field('start', $._expression),
          ',',
          field('stop', $._expression),
          optional(seq(',', field('step', $._expression))),
          'do',
          optional(field('body', $.block)),
          'end',
        ),
      ),
    ),

    _for_binding: $ => choice(
      $.identifier,
      $.wildcard_pattern,
      $.tuple_pattern,
      $.record_pattern,
    ),

    loop_statement: $ => seq('loop', optional(field('body', $.block)), 'end'),
    do_statement: $ => seq('do', optional(field('body', $.block)), 'end'),
    unsafe_statement: $ => seq('unsafe', optional(field('body', $.block)), 'end'),
    break_statement: $ => 'break',
    continue_statement: $ => 'continue',

    return_statement: $ => seq(
      'return',
      optional(field('value', commaSep1($._expression))),
    ),

    block: $ => repeat1($._statement),

    // ==================================================================
    // Match expressions
    // ==================================================================

    match_expression: $ => seq(
      'match',
      field('value', $._expression),
      repeat($.match_arm),
      'end',
    ),

    match_arm: $ => seq(
      optional('case'),
      field('pattern', $._pattern),
      optional($.match_guard),
      '=>',
      field('body', choice($._expression, seq('do', optional($.block), 'end'))),
    ),

    match_guard: $ => seq('if', field('condition', $._expression)),

    // ==================================================================
    // Patterns
    // ==================================================================

    _pattern: $ => choice(
      $.or_pattern,
      $.as_pattern,
      $._pattern_atom,
    ),

    or_pattern: $ => prec.left(1, seq($._pattern, '|', $._pattern)),

    as_pattern: $ => prec(2, seq($._pattern, 'as', $.identifier)),

    _pattern_atom: $ => choice(
      $.wildcard_pattern,
      $.variant_pattern,
      $.struct_pattern,
      $.tuple_pattern,
      $.array_pattern,
      $.record_pattern,
      $.scoped_identifier,
      $.identifier,
      $.number,
      $.string,
      $.raw_string,
      $.char_literal,
      $.boolean,
      $.null,
      $.nil,
    ),

    wildcard_pattern: $ => '_',

    variant_pattern: $ => seq(
      field('name', choice($.identifier, $.scoped_identifier)),
      '(',
      optional(seq(commaSep1($._pattern), optional(','))),
      ')',
    ),

    struct_pattern: $ => seq(
      field('type', choice($.identifier, $.scoped_identifier)),
      '{',
      optional(seq(commaSep1($.struct_pattern_field), optional(','))),
      '}',
    ),

    struct_pattern_field: $ => seq(
      field('name', $.identifier),
      optional(seq(':', field('pattern', $._pattern))),
    ),

    tuple_pattern: $ => seq(
      '(',
      optional(seq(commaSep1(choice($._pattern, $.rest_pattern)), optional(','))),
      ')',
    ),

    array_pattern: $ => seq(
      '[',
      optional(seq(commaSep1(choice($._pattern, $.rest_pattern)), optional(','))),
      ']',
    ),

    rest_pattern: $ => seq('..', optional($.identifier)),

    record_pattern: $ => seq(
      '{',
      optional(seq(commaSep1($.record_pattern_field), optional(','))),
      '}',
    ),

    record_pattern_field: $ => seq(
      field('name', $.identifier),
      optional(seq(':', field('pattern', $._pattern))),
    ),

    // ==================================================================
    // Types
    // ==================================================================

    _type: $ => choice(
      $.primitive_type,
      $.type_identifier,
      $.optional_type,
      $.array_type,
      $.generic_type,
      $.tuple_type,
      $.reference_type,
      $.pointer_type,
      $.borrow_type,
      $.mut_type,
      $.move_type,
    ),

    type_identifier: $ => alias($.identifier, 'type_identifier'),

    primitive_type: $ => choice(
      'Int', 'UInt', 'Float', 'Double', 'Bool', 'String', 'Char', 'Byte',
      'i8', 'i16', 'i32', 'i64', 'i128',
      'u8', 'u16', 'u32', 'u64', 'u128',
      'f32', 'f64', 'isize', 'usize', 'Self',
    ),

    optional_type: $ => prec.left(1, seq($._type, repeat1('?'))),

    array_type: $ => seq('[', field('element', $._type), ']'),

    generic_type: $ => seq(
      field('name', $.type_identifier),
      '<',
      commaSep1(field('type', $._type)),
      optional(','),
      '>',
    ),

    tuple_type: $ => prec(1, seq('(', $._type, ',', commaSep1($._type), optional(','), ')')),

    reference_type: $ => prec.right(2, seq('&', optional('mut'), $._type)),

    pointer_type: $ => prec.right(1, seq('*', choice('const', 'mut'), $._type)),

    borrow_type: $ => prec.right(1, seq('borrow', $._type)),

    mut_type: $ => prec.right(1, seq('mut', $._type)),

    move_type: $ => prec.right(1, seq('move', $._type)),

    // ==================================================================
    // Expressions
    // ==================================================================

    _expression: $ => choice(
      $.binary_expression,
      $.unary_expression,
      $.await_expression,
      $.spawn_expression,
      $.as_expression,
      $.call_expression,
      $.colon_method_call_expression,
      $.field_expression,
      $.index_expression,
      $.try_expression,
      $.unwrap_expression,
      $.struct_expression,
      $.spread_expression,
      $.function_expression,
      $.short_closure,
      $.if_expression,
      $.match_expression,
      $.array_expression,
      $.record_expression,
      $.tuple_expression,
      $.parenthesized_expression,
      $.number,
      $.string,
      $.raw_string,
      $.char_literal,
      $.boolean,
      $.null,
      $.nil,
      $.scoped_identifier,
      $.identifier,
      $.self_expression,
    ),

    binary_expression: $ => {
      const operators = [
        ['|>', PREC.PIPE], ['<|', PREC.PIPE],
        ['or', PREC.OR],
        ['??', PREC.COALESCE],
        ['and', PREC.AND],
        ['==', PREC.COMPARE], ['!=', PREC.COMPARE], ['~=', PREC.COMPARE],
        ['<', PREC.COMPARE], ['<=', PREC.COMPARE],
        ['>', PREC.COMPARE], ['>=', PREC.COMPARE],
        ['..', PREC.RANGE], ['..=', PREC.RANGE],
        ['|', PREC.BITWISE_OR],
        ['^', PREC.BITWISE_XOR],
        ['&', PREC.BITWISE_AND],
        ['<<', PREC.SHIFT], ['>>', PREC.SHIFT],
        ['+', PREC.ADD], ['-', PREC.ADD],
        ['*', PREC.MULTIPLY], ['/', PREC.MULTIPLY], ['%', PREC.MULTIPLY],
      ];
      return choice(...operators.map(([operator, precedence]) =>
        prec.left(precedence, seq(
          field('left', $._expression),
          field('operator', operator),
          field('right', $._expression),
        )),
      ));
    },

    unary_expression: $ => prec(PREC.UNARY, seq(
      field('operator', choice('not', '-', '!', '~')),
      field('operand', $._expression),
    )),

    await_expression: $ => prec(PREC.UNARY, seq('await', field('value', $._expression))),

    spawn_expression: $ => prec(PREC.UNARY, seq('spawn', field('value', $._expression))),

    as_expression: $ => prec.left(PREC.CAST, seq(
      field('value', $._expression),
      'as',
      field('type', $._type),
    )),

    call_expression: $ => prec.left(PREC.POSTFIX, seq(
      field('function', $._expression),
      field('arguments', $.arguments),
    )),

    arguments: $ => seq(
      '(',
      optional(seq(commaSep1($._argument), optional(','))),
      ')',
    ),

    _argument: $ => choice(
      $.named_argument,
      $._expression,
    ),

    named_argument: $ => prec(1, seq(
      field('name', $.identifier),
      choice(':', '='),
      field('value', $._expression),
    )),

    colon_method_call_expression: $ => prec.left(PREC.POSTFIX, seq(
      field('object', $._expression),
      ':',
      field('method', $.identifier),
      field('arguments', $.arguments),
    )),

    field_expression: $ => prec.left(PREC.POSTFIX, seq(
      field('object', $._expression),
      '.',
      field('field', $.identifier),
    )),

    index_expression: $ => prec.left(PREC.POSTFIX, seq(
      field('object', $._expression),
      '[',
      field('index', $._expression),
      ']',
    )),

    try_expression: $ => prec.left(PREC.POSTFIX, seq(
      field('value', $._expression),
      '?',
    )),

    unwrap_expression: $ => prec.left(PREC.POSTFIX, seq(
      field('value', $._expression),
      '!',
    )),

    struct_expression: $ => prec.left(PREC.POSTFIX, seq(
      field('type', $._expression),
      '{',
      repeat(choice($._record_field, ',')),
      '}',
    )),

    spread_expression: $ => prec(PREC.UNARY, seq('...', field('value', $._expression))),

    function_expression: $ => prec.right(seq(
      'fn',
      optional(field('type_parameters', $.type_parameters)),
      field('parameters', $.parameters),
      optional($.return_type),
      optional(field('body', $.block)),
      'end',
    )),

    short_closure: $ => prec.right(seq(
      '|',
      optional(commaSep1(seq(
        $.identifier,
        optional(seq(':', $._type)),
      ))),
      '|',
      field('body', $._expression),
    )),

    array_expression: $ => seq(
      '[',
      optional(seq(
        commaSep1($._expression),
        optional(','),
      )),
      ']',
    ),

    record_expression: $ => seq(
      '{',
      repeat(choice($._record_field, ',')),
      '}',
    ),

    _record_field: $ => choice(
      $.spread_expression,
      $.record_field,
    ),

    record_field: $ => prec(1, choice(
      seq(field('name', $.identifier), ':', field('value', $._expression)),
      seq(field('name', $.identifier), '=', field('value', $._expression)),
    )),

    tuple_expression: $ => prec(1, seq(
      '(',
      $._expression,
      ',',
      commaSep1($._expression),
      optional(','),
      ')',
    )),

    parenthesized_expression: $ => seq('(', $._expression, ')'),

    self_expression: $ => 'self',

    scoped_identifier: $ => prec.right(seq(
      $.identifier,
      repeat1(seq('::', $.identifier)),
    )),

    // ==================================================================
    // Literals
    // ==================================================================

    number: $ => choice($.integer, $.float),

    integer: $ => choice(
      /[0-9][0-9_]*(_[A-Za-z][A-Za-z0-9_]*)?/,
      /0[xX][0-9a-fA-F_]+/,
      /0[bB][01_]+/,
      /0[oO][0-7_]+/,
    ),

    float: $ => choice(
      /[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9]+)?(_[A-Za-z][A-Za-z0-9_]*)?/,
      /[0-9][0-9_]*[eE][+-]?[0-9]+/,
    ),

    boolean: $ => choice('true', 'false'),

    null: $ => 'null',

    nil: $ => 'nil',

    string: $ => seq(
      '"',
      repeat(choice(
        $.escape_sequence,
        $.string_interpolation,
        $._string_text_chunk,
        $._string_dollar,
      )),
      '"',
    ),

    string_interpolation: $ => seq(
      '${',
      field('value', $._expression),
      '}',
    ),

    escape_sequence: $ => /\\./,

    raw_string: $ => /`[^`]*`/,

    char_literal: $ => /'([^'\\\n]|\\.)'/,

    // ==================================================================
    // Comments (extras)
    // ==================================================================

    comment: $ => token(seq('--', optional(seq(/[^\[\n]/, repeat(/[^\n]/))))),

    block_comment: $ => /\-\-\[\[([^\]]|\][^\]])*\]\]/,

    doc_comment: $ => token(prec(1, /---[^\n]*/)),

    _string_text_chunk: $ => token(prec(1, /[^"\\$]+/)),

    _string_dollar: $ => /\$/,

    identifier: $ => /[A-Za-z_][A-Za-z0-9_]*/,
  },
});
