// Owner: PascalElixir / axolrs (GitHub org)
// File: unit tests for command construction, server resolution, grammar, and manifest.

import * as assert from 'assert';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';
import {
  buildCommandArgs,
  commandIdFor,
  emitRustArgs,
  formatCommandLine,
  getToolCommandSpec,
  quoteShellArg,
  statusLabelFor,
  terminalNameFor,
  TOOL_COMMANDS,
} from '../commands';
import {
  DEFAULT_SERVER_FALLBACK,
  DEFAULT_SERVER_PATH,
  resolveExecutable,
  resolveServerOptions,
  splitCommandLine,
} from '../server';

interface GrammarFile {
  name?: string;
  scopeName?: string;
  patterns?: unknown[];
  repository?: Record<string, GrammarRule>;
}

interface GrammarRule {
  name?: string;
  match?: string;
  begin?: string;
  end?: string;
  patterns?: GrammarRule[];
}

interface ManifestCommand {
  command?: string;
  title?: string;
}

interface Manifest {
  version?: string;
  license?: string;
  main?: string;
  contributes?: {
    commands?: ManifestCommand[];
    languages?: { id?: string; extensions?: string[]; configuration?: string }[];
    grammars?: { language?: string; scopeName?: string; path?: string }[];
    configuration?: { properties?: Record<string, { type?: string; default?: unknown }> };
  };
}

const extensionRoot = path.join(__dirname, '..', '..');

/** Reads and parses a JSON file relative to the extension root. */
function readJson(relativePath: string): unknown {
  const filePath = path.join(extensionRoot, relativePath);
  const raw = fs.readFileSync(filePath, 'utf8');
  return JSON.parse(raw);
}

/** Creates a temporary directory containing a fake executable with the given name. */
function makeFakeExecutableDir(name: string): string {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'axolcode-test-'));
  const filePath = path.join(dir, name);
  fs.writeFileSync(filePath, '#!/bin/sh\nexit 0\n', { mode: 0o755 });
  fs.chmodSync(filePath, 0o755);
  return dir;
}

/** Recursively collects every match regex in a grammar rule tree. */
function collectMatches(rule: GrammarRule, matches: string[]): string[] {
  if (typeof rule.match === 'string') {
    matches.push(rule.match);
  }
  for (const child of rule.patterns ?? []) {
    collectMatches(child, matches);
  }
  return matches;
}

describe('tool command construction', () => {
  const expectedArgs: Record<string, string[]> = {
    run: ['bucket', 'run'],
    build: ['bucket', 'build', '--release'],
    test: ['bucket', 'test'],
    fmt: ['bucket', 'fmt'],
    lint: ['bucket', 'lint'],
    fix: ['bucket', 'fix'],
    doc: ['bucket', 'doc'],
    interpretedRun: ['bucket', 'run', '--interpret'],
    interpretedREPL: ['axol-hot-runner', 'repl'],
  };

  it('covers all nine tool commands', () => {
    assert.deepStrictEqual([...TOOL_COMMANDS].sort(), Object.keys(expectedArgs).sort());
  });

  it('builds the exact argv for every tool command', () => {
    for (const kind of TOOL_COMMANDS) {
      assert.deepStrictEqual(buildCommandArgs(kind), expectedArgs[kind], `argv mismatch for ${kind}`);
    }
  });

  it('returns terminal names and status labels for every tool command', () => {
    for (const kind of TOOL_COMMANDS) {
      const spec = getToolCommandSpec(kind);
      assert.strictEqual(spec.terminalName, terminalNameFor(kind));
      assert.strictEqual(spec.statusLabel, statusLabelFor(kind));
      assert.ok(spec.terminalName.length > 0, `empty terminal name for ${kind}`);
      assert.ok(spec.statusLabel.length > 0, `empty status label for ${kind}`);
    }
  });

  it('derives VS Code command ids from the tool command kind', () => {
    assert.strictEqual(commandIdFor('run'), 'axolcode.run');
    assert.strictEqual(commandIdFor('interpretedREPL'), 'axolcode.interpretedREPL');
  });

  it('builds the argv that emits generated Rust for a document', () => {
    assert.deepStrictEqual(emitRustArgs('/proj/src/main.axol'), ['bucket', 'emit-rust', '/proj/src/main.axol']);
  });
});

describe('shell command line formatting', () => {
  it('leaves plain arguments unquoted', () => {
    assert.strictEqual(formatCommandLine(['bucket', 'run']), 'bucket run');
    assert.strictEqual(formatCommandLine(['bucket', 'build', '--release']), 'bucket build --release');
  });

  it('quotes arguments containing spaces', () => {
    assert.strictEqual(formatCommandLine(['bucket', 'emit-rust', 'my file.axol']), "bucket emit-rust 'my file.axol'");
  });

  it('quotes and escapes arguments containing single quotes', () => {
    assert.strictEqual(quoteShellArg("don't.axol"), "'don'\\''t.axol'");
  });

  it('quotes empty arguments', () => {
    assert.strictEqual(quoteShellArg(''), "''");
  });

  it('quotes arguments containing shell metacharacters', () => {
    assert.strictEqual(quoteShellArg('a$b'), "'a$b'");
    assert.strictEqual(quoteShellArg('a;b'), "'a;b'");
  });
});

describe('command line splitting', () => {
  it('splits a bare command name', () => {
    assert.deepStrictEqual(splitCommandLine('axol-analyzer'), ['axol-analyzer']);
  });

  it('splits the fallback command into tokens', () => {
    assert.deepStrictEqual(splitCommandLine('bucket gills --stdio'), ['bucket', 'gills', '--stdio']);
  });

  it('collapses repeated whitespace', () => {
    assert.deepStrictEqual(splitCommandLine('  bucket   run  '), ['bucket', 'run']);
  });

  it('keeps quoted segments as single tokens', () => {
    assert.deepStrictEqual(splitCommandLine('"axol analyzer" --stdio'), ['axol analyzer', '--stdio']);
    assert.deepStrictEqual(splitCommandLine("'my dir/tool' run"), ['my dir/tool', 'run']);
  });

  it('respects escapes inside double quotes', () => {
    assert.deepStrictEqual(splitCommandLine('"a\\"b" c'), ['a"b', 'c']);
  });

  it('returns no tokens for an empty line', () => {
    assert.deepStrictEqual(splitCommandLine('   '), []);
  });
});

describe('executable resolution', () => {
  it('finds a command on PATH', () => {
    const dir = makeFakeExecutableDir('axol-analyzer');
    const resolved = resolveExecutable('axol-analyzer', { PATH: dir }, 'linux');
    assert.strictEqual(resolved, path.join(dir, 'axol-analyzer'));
    fs.rmSync(dir, { recursive: true, force: true });
  });

  it('returns undefined for a command that is not on PATH', () => {
    const dir = makeFakeExecutableDir('bucket');
    const resolved = resolveExecutable('axol-analyzer', { PATH: dir }, 'linux');
    assert.strictEqual(resolved, undefined);
    fs.rmSync(dir, { recursive: true, force: true });
  });

  it('skips non-executable files', () => {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'axolcode-test-'));
    fs.writeFileSync(path.join(dir, 'axol-analyzer'), 'not executable', { mode: 0o644 });
    const resolved = resolveExecutable('axol-analyzer', { PATH: dir }, 'linux');
    assert.strictEqual(resolved, undefined);
    fs.rmSync(dir, { recursive: true, force: true });
  });

  it('searches every PATH entry in order', () => {
    const firstDir = makeFakeExecutableDir('other-tool');
    const secondDir = makeFakeExecutableDir('bucket');
    const resolved = resolveExecutable('bucket', { PATH: `${firstDir}${path.delimiter}${secondDir}` }, 'linux');
    assert.strictEqual(resolved, path.join(secondDir, 'bucket'));
    fs.rmSync(firstDir, { recursive: true, force: true });
    fs.rmSync(secondDir, { recursive: true, force: true });
  });

  it('uses PATHEXT extensions on Windows', () => {
    const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'axolcode-test-'));
    fs.writeFileSync(path.join(dir, 'axol-analyzer.cmd'), '@echo off', { mode: 0o755 });
    const resolved = resolveExecutable('axol-analyzer', { PATH: dir, PATHEXT: '.com;.exe;.bat;.cmd' }, 'win32');
    assert.strictEqual(resolved, path.join(dir, 'axol-analyzer.cmd'));
    fs.rmSync(dir, { recursive: true, force: true });
  });
});

describe('server command resolution', () => {
  it('defaults match the Gills toolchain', () => {
    assert.strictEqual(DEFAULT_SERVER_PATH, 'axol-analyzer');
    assert.strictEqual(DEFAULT_SERVER_FALLBACK, 'bucket gills --stdio');
  });

  it('uses the primary command when it is on PATH', () => {
    const dir = makeFakeExecutableDir('axol-analyzer');
    const resolved = resolveServerOptions('axol-analyzer', 'bucket gills --stdio', { PATH: dir }, 'linux');
    assert.deepStrictEqual(resolved, { command: path.join(dir, 'axol-analyzer'), args: [] });
    fs.rmSync(dir, { recursive: true, force: true });
  });

  it('falls back to the fallback command when the primary is missing', () => {
    const dir = makeFakeExecutableDir('bucket');
    const resolved = resolveServerOptions('axol-analyzer', 'bucket gills --stdio', { PATH: dir }, 'linux');
    assert.deepStrictEqual(resolved, { command: path.join(dir, 'bucket'), args: ['gills', '--stdio'] });
    fs.rmSync(dir, { recursive: true, force: true });
  });

  it('keeps the primary command when nothing resolves so spawn errors surface', () => {
    const dir = makeFakeExecutableDir('unrelated');
    const resolved = resolveServerOptions('axol-analyzer --stdio', 'bucket gills --stdio', { PATH: dir }, 'linux');
    assert.deepStrictEqual(resolved, { command: 'axol-analyzer', args: ['--stdio'] });
    fs.rmSync(dir, { recursive: true, force: true });
  });

  it('uses the fallback command without arguments when it is bare', () => {
    const dir = makeFakeExecutableDir('custom-server');
    const resolved = resolveServerOptions('axol-analyzer', 'custom-server', { PATH: dir }, 'linux');
    assert.deepStrictEqual(resolved, { command: path.join(dir, 'custom-server'), args: [] });
    fs.rmSync(dir, { recursive: true, force: true });
  });
});

describe('TextMate grammar', () => {
  const grammar = readJson('syntaxes/axol.tmLanguage.json') as GrammarFile;

  it('declares the Axolotl name and source.axol scope', () => {
    assert.strictEqual(grammar.name, 'Axolotl');
    assert.strictEqual(grammar.scopeName, 'source.axol');
  });

  it('has top-level patterns and a populated repository', () => {
    assert.ok(Array.isArray(grammar.patterns));
    assert.ok((grammar.patterns ?? []).length >= 10);
    assert.ok(Object.keys(grammar.repository ?? {}).length >= 10);
  });

  it('matches every keyword of the language', () => {
    const keywordMatches = [
      ...collectMatches((grammar.repository ?? {}).keywords ?? {}, []),
      ...collectMatches((grammar.repository ?? {})['function-definitions'] ?? {}, []),
    ].join(' ');
    const keywords = [
      'fn', 'let', 'var', 'if', 'then', 'else', 'elseif', 'end', 'while', 'for', 'do', 'return',
      'match', 'case', 'struct', 'enum', 'interface', 'impl', 'const', 'type', 'use', 'async',
      'await', 'spawn', 'break', 'continue', 'repeat', 'until', 'loop', 'pub', 'extern', 'unsafe',
    ];
    for (const keyword of keywords) {
      assert.ok(keywordMatches.includes(keyword), `grammar is missing keyword '${keyword}'`);
    }
  });

  it('supports line and block comments', () => {
    const comments = collectMatches((grammar.repository ?? {}).comments ?? {}, []);
    const joined = comments.join(' ');
    assert.ok(joined.includes('--'), 'line comment marker missing');
    assert.ok((grammar.repository ?? {}).comments?.patterns?.some((rule) => rule.begin === '--\\[\\['), 'block comment begin missing');
    assert.ok((grammar.repository ?? {}).comments?.patterns?.some((rule) => rule.end === '\\]\\]'), 'block comment end missing');
  });

  it('supports double-quoted strings with ${...} interpolation', () => {
    const strings = (grammar.repository ?? {}).strings;
    assert.strictEqual(strings?.begin, '"');
    assert.strictEqual(strings?.end, '"');
    const interpolations = (grammar.repository ?? {})['string-interpolations'];
    assert.strictEqual(interpolations?.begin, '\\$\\{');
    assert.strictEqual(interpolations?.end, '\\}');
    assert.ok((grammar.repository ?? {})['string-escapes']?.match?.includes('u\\{'), 'unicode escape missing');
  });

  it('supports booleans, nil, and numbers', () => {
    const booleans = collectMatches((grammar.repository ?? {}).booleans ?? {}, []).join(' ');
    assert.ok(booleans.includes('true'));
    assert.ok(booleans.includes('false'));
    assert.ok(booleans.includes('nil'));
    const numbers = (grammar.repository ?? {}).numbers?.patterns ?? [];
    assert.ok(numbers.length >= 2, 'integer and float patterns expected');
  });
});

describe('extension manifest', () => {
  const manifest = readJson('package.json') as Manifest;
  const commandIds = (manifest.contributes?.commands ?? []).map((entry) => entry.command ?? '');

  it('pins the 0.1.0 version with an MIT license and the webpack bundle entry', () => {
    assert.strictEqual(manifest.version, '0.1.0');
    assert.strictEqual(manifest.license, 'MIT');
    assert.strictEqual(manifest.main, './dist/extension.js');
  });

  it('registers unique command ids with titles', () => {
    const requiredIds = [
      'axolcode.run',
      'axolcode.build',
      'axolcode.test',
      'axolcode.fmt',
      'axolcode.lint',
      'axolcode.fix',
      'axolcode.doc',
      'axolcode.interpretedRun',
      'axolcode.interpretedREPL',
      'axolcode.showGeneratedRust',
      'axolcode.buildStatus',
    ];
    assert.deepStrictEqual([...commandIds].sort(), [...new Set(commandIds)].sort(), 'command ids must be unique');
    for (const id of requiredIds) {
      assert.ok(commandIds.includes(id), `command '${id}' missing from contributes.commands`);
      const entry = (manifest.contributes?.commands ?? []).find((item) => item.command === id);
      assert.ok(entry?.title && entry.title.length > 0, `command '${id}' needs a title`);
    }
    assert.ok(!commandIds.includes('axolcode.helloWorld'), 'placeholder command must be gone');
  });

  it('declares the axol language with the .axol extension and a configuration file', () => {
    const language = (manifest.contributes?.languages ?? []).find((item) => item.id === 'axol');
    assert.ok(language, 'axol language contribution missing');
    assert.deepStrictEqual(language?.extensions, ['.axol']);
    assert.ok(language?.configuration && fs.existsSync(path.join(extensionRoot, language.configuration)));
  });

  it('declares the grammar bound to source.axol with an existing file', () => {
    const grammar = (manifest.contributes?.grammars ?? []).find((item) => item.language === 'axol');
    assert.strictEqual(grammar?.scopeName, 'source.axol');
    assert.ok(grammar?.path && fs.existsSync(path.join(extensionRoot, grammar.path)));
  });

  it('declares the server and run settings with defaults', () => {
    const properties = manifest.contributes?.configuration?.properties ?? {};
    assert.strictEqual(properties['axolcode.server.path']?.type, 'string');
    assert.strictEqual(properties['axolcode.server.path']?.default, 'axol-analyzer');
    assert.strictEqual(properties['axolcode.server.fallbackCommand']?.type, 'string');
    assert.strictEqual(properties['axolcode.server.fallbackCommand']?.default, 'bucket gills --stdio');
    assert.strictEqual(properties['axolcode.run.saveOnRun']?.type, 'boolean');
    assert.strictEqual(properties['axolcode.run.saveOnRun']?.default, false);
  });
});

describe('language configuration', () => {
  it('wires -- line comments and --[[ ]] block comments', () => {
    const config = readJson('language-configuration.json') as { comments?: { lineComment?: string; blockComment?: string[] } };
    assert.strictEqual(config.comments?.lineComment, '--');
    assert.deepStrictEqual(config.comments?.blockComment, ['--[[', ']]']);
  });
});
