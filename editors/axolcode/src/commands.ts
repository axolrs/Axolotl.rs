// Owner: PascalElixir / axolrs (GitHub org)
// File: bucket tool command definitions and shell command construction helpers.

export type ToolCommand =
  | 'run'
  | 'build'
  | 'test'
  | 'fmt'
  | 'lint'
  | 'fix'
  | 'doc'
  | 'interpretedRun'
  | 'interpretedREPL';

export interface ToolCommandSpec {
  readonly kind: ToolCommand;
  readonly argv: string[];
  readonly terminalName: string;
  readonly statusLabel: string;
}

const TOOL_COMMAND_SPECS: Readonly<Record<ToolCommand, ToolCommandSpec>> = {
  run: { kind: 'run', argv: ['bucket', 'run'], terminalName: 'axol run', statusLabel: 'running' },
  build: { kind: 'build', argv: ['bucket', 'build', '--release'], terminalName: 'axol build', statusLabel: 'building' },
  test: { kind: 'test', argv: ['bucket', 'test'], terminalName: 'axol test', statusLabel: 'testing' },
  fmt: { kind: 'fmt', argv: ['bucket', 'fmt'], terminalName: 'axol fmt', statusLabel: 'formatting' },
  lint: { kind: 'lint', argv: ['bucket', 'lint'], terminalName: 'axol lint', statusLabel: 'linting' },
  fix: { kind: 'fix', argv: ['bucket', 'fix'], terminalName: 'axol fix', statusLabel: 'fixing' },
  doc: { kind: 'doc', argv: ['bucket', 'doc'], terminalName: 'axol doc', statusLabel: 'building docs' },
  interpretedRun: { kind: 'interpretedRun', argv: ['bucket', 'run', '--interpret'], terminalName: 'axol run (interpreted)', statusLabel: 'running (interpreted)' },
  interpretedREPL: { kind: 'interpretedREPL', argv: ['axol-hot-runner', 'repl'], terminalName: 'axol repl', statusLabel: 'repl' },
};

export const TOOL_COMMANDS: readonly ToolCommand[] = Object.keys(TOOL_COMMAND_SPECS) as ToolCommand[];

/** Returns the full spec (argv, terminal name, status label) for a tool command. */
export function getToolCommandSpec(kind: ToolCommand): ToolCommandSpec {
  return TOOL_COMMAND_SPECS[kind];
}

/** Returns the raw argv for a tool command, for example ['bucket', 'build', '--release']. */
export function buildCommandArgs(kind: ToolCommand): string[] {
  return [...TOOL_COMMAND_SPECS[kind].argv];
}

/** Returns the display name of the terminal that runs the given tool command. */
export function terminalNameFor(kind: ToolCommand): string {
  return TOOL_COMMAND_SPECS[kind].terminalName;
}

/** Returns the status-bar label used while the given tool command is running. */
export function statusLabelFor(kind: ToolCommand): string {
  return TOOL_COMMAND_SPECS[kind].statusLabel;
}

/** Returns the VS Code command id for a tool command, for example 'axolcode.run'. */
export function commandIdFor(kind: ToolCommand): string {
  return `axolcode.${kind}`;
}

const SAFE_ARG_PATTERN = /^[A-Za-z0-9_@%+=:,./-]+$/;

/** Wraps a single argument in single quotes when it contains shell metacharacters. */
export function quoteShellArg(arg: string): string {
  if (arg.length === 0) {
    return "''";
  }
  if (SAFE_ARG_PATTERN.test(arg)) {
    return arg;
  }
  return `'${arg.replace(/'/g, "'\\''")}'`;
}

/** Joins an argv array into one shell command line, quoting arguments when needed. */
export function formatCommandLine(args: string[]): string {
  return args.map((arg) => quoteShellArg(arg)).join(' ');
}

/** Returns the argv that emits the generated Rust for an .axol source file. */
export function emitRustArgs(documentPath: string): string[] {
  return ['bucket', 'emit-rust', documentPath];
}
