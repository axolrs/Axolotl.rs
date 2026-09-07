// Owner: PascalElixir / axolrs (GitHub org)
// File: axolcode extension entry point - activation, LSP client, commands, status bar.

import * as vscode from 'vscode';
import { execFile } from 'node:child_process';
import * as util from 'node:util';
import type { LanguageClient } from 'vscode-languageclient/node';
import {
  buildCommandArgs,
  commandIdFor,
  emitRustArgs,
  formatCommandLine,
  getToolCommandSpec,
  TOOL_COMMANDS,
  type ToolCommand,
} from './commands';
import { createLanguageClient } from './client';
import { GENERATED_RUST_SCHEME, GeneratedRustProvider, generatedRustUri } from './generated';
import { DEFAULT_SERVER_FALLBACK, DEFAULT_SERVER_PATH, resolveServerOptions } from './server';
import { AxolStatusBar } from './status';

const execFileAsync = util.promisify(execFile);
const MAX_EMIT_BUFFER_BYTES = 32 * 1024 * 1024;

let languageClient: LanguageClient | undefined;
let statusBar: AxolStatusBar | undefined;
let generatedProvider: GeneratedRustProvider | undefined;
let serverOnline = false;
const busyTerminals = new Set<vscode.Terminal>();

/** Activates the extension by starting the language client and registering all commands. */
export function activate(context: vscode.ExtensionContext): void {
  const outputChannel = vscode.window.createOutputChannel('AxolCode', { log: true });
  statusBar = new AxolStatusBar();
  generatedProvider = new GeneratedRustProvider();
  context.subscriptions.push(
    outputChannel,
    statusBar,
    generatedProvider,
    vscode.workspace.registerTextDocumentContentProvider(GENERATED_RUST_SCHEME, generatedProvider),
    vscode.window.onDidCloseTerminal((closed) => handleTerminalClosed(closed)),
  );
  registerToolCommands(context, outputChannel);
  registerAuxiliaryCommands(context, outputChannel);
  startLanguageClient(outputChannel);
}

/** Stops the Gills language client when the extension is deactivated. */
export function deactivate(): Thenable<void> | undefined {
  statusBar?.setStopped();
  return languageClient?.stop();
}

/** Marks a finished tool terminal and updates the status bar with its exit state. */
function handleTerminalClosed(closed: vscode.Terminal): void {
  if (!busyTerminals.delete(closed)) {
    return;
  }
  const exitCode = closed.exitStatus?.code;
  if (exitCode !== undefined && exitCode !== 0) {
    statusBar?.setFailed(`failed (exit ${exitCode})`);
    return;
  }
  restoreIdleStatus();
}

/** Restores the idle status bar state that matches the language server state. */
function restoreIdleStatus(): void {
  if (serverOnline) {
    statusBar?.setReady();
  } else {
    statusBar?.setServerMissing();
  }
}

/** Registers every terminal-based bucket and REPL command. */
function registerToolCommands(context: vscode.ExtensionContext, outputChannel: vscode.LogOutputChannel): void {
  for (const kind of TOOL_COMMANDS) {
    const disposable = vscode.commands.registerCommand(commandIdFor(kind), () => runToolCommand(kind, outputChannel));
    context.subscriptions.push(disposable);
  }
}

/** Registers the generated-Rust and status-bar commands. */
function registerAuxiliaryCommands(context: vscode.ExtensionContext, outputChannel: vscode.LogOutputChannel): void {
  const showGeneratedRust = vscode.commands.registerCommand('axolcode.showGeneratedRust', () =>
    showGeneratedRustForActiveDocument(outputChannel),
  );
  const buildStatus = vscode.commands.registerCommand('axolcode.buildStatus', () => statusBar?.toggle());
  context.subscriptions.push(showGeneratedRust, buildStatus);
}

/** Runs a bucket tool command in a fresh terminal rooted at the first workspace folder. */
async function runToolCommand(kind: ToolCommand, outputChannel: vscode.LogOutputChannel): Promise<void> {
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (folder === undefined) {
    vscode.window.showErrorMessage('AxolCode: open a workspace folder before running Axolotl commands.');
    return;
  }
  if (kind === 'run' && shouldSaveOnRun()) {
    await vscode.workspace.saveAll(false);
  }
  const spec = getToolCommandSpec(kind);
  outputChannel.info(`Running ${spec.argv.join(' ')} in ${folder.uri.fsPath}`);
  const terminal = vscode.window.createTerminal({ name: spec.terminalName, cwd: folder.uri.fsPath });
  busyTerminals.add(terminal);
  statusBar?.setBusy(spec.statusLabel);
  terminal.show(true);
  terminal.sendText(formatCommandLine(buildCommandArgs(kind)), true);
}

/** Reports whether the run command should save all files before executing. */
function shouldSaveOnRun(): boolean {
  return vscode.workspace.getConfiguration('axolcode').get<boolean>('run.saveOnRun', false);
}

/** Emits the Rust for the active .axol document and shows it in a read-only editor. */
async function showGeneratedRustForActiveDocument(outputChannel: vscode.LogOutputChannel): Promise<void> {
  const document = vscode.window.activeTextEditor?.document;
  if (document === undefined || (document.languageId !== 'axol' && !document.fileName.endsWith('.axol'))) {
    vscode.window.showErrorMessage('AxolCode: open an .axol file to show its generated Rust.');
    return;
  }
  const folder = vscode.workspace.workspaceFolders?.[0];
  if (folder === undefined) {
    vscode.window.showErrorMessage('AxolCode: open a workspace folder before emitting Rust.');
    return;
  }
  statusBar?.setBusy('emit-rust');
  const argv = emitRustArgs(document.fileName);
  try {
    const { stdout } = await execFileAsync(argv[0], argv.slice(1), {
      cwd: folder.uri.fsPath,
      encoding: 'utf8',
      maxBuffer: MAX_EMIT_BUFFER_BYTES,
    });
    outputChannel.info(`Emitted Rust for ${document.fileName} (${stdout.length} characters).`);
    statusBar?.setReady();
    await openGeneratedRust(stdout, document.fileName, outputChannel);
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    outputChannel.error(`emit-rust failed: ${message}`);
    vscode.window.showErrorMessage(`AxolCode: bucket emit-rust failed: ${message}`);
    statusBar?.setFailed('emit-rust failed');
  }
}

/** Opens emitted Rust content as a read-only virtual document with Rust coloring. */
async function openGeneratedRust(content: string, sourcePath: string, outputChannel: vscode.LogOutputChannel): Promise<void> {
  const uri = generatedRustUri(sourcePath);
  generatedProvider?.set(uri, content);
  const textDocument = await vscode.workspace.openTextDocument(uri);
  try {
    await vscode.languages.setTextDocumentLanguage(textDocument, 'rust');
  } catch (err) {
    outputChannel.warn(`Rust language not available, opening as plain text: ${String(err)}`);
  }
  await vscode.window.showTextDocument(textDocument, { preview: true });
}

/** Resolves the server command from settings and starts the Gills language client. */
function startLanguageClient(outputChannel: vscode.LogOutputChannel): void {
  const config = vscode.workspace.getConfiguration('axolcode');
  const primary = config.get<string>('server.path', DEFAULT_SERVER_PATH);
  const fallback = config.get<string>('server.fallbackCommand', DEFAULT_SERVER_FALLBACK);
  const cwd = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  const server = resolveServerOptions(primary, fallback, process.env, process.platform);
  outputChannel.info(`Starting Gills: ${server.command} ${server.args.join(' ')}`);
  languageClient = createLanguageClient(server, outputChannel, cwd);
  languageClient
    .start()
    .then(() => {
      serverOnline = true;
      statusBar?.setReady();
    })
    .catch((err: unknown) => {
      serverOnline = false;
      const message = err instanceof Error ? err.message : String(err);
      outputChannel.error(`Gills failed to start: ${message}`);
      statusBar?.setServerMissing();
      vscode.window.showWarningMessage(
        `AxolCode: could not start the Gills language server (${message}). Check the axolcode.server.path setting.`,
      );
    });
}
