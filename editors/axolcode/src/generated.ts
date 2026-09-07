// Owner: PascalElixir / axolrs (GitHub org)
// File: read-only virtual documents that show Rust generated from Axolotl sources.

import * as vscode from 'vscode';

export const GENERATED_RUST_SCHEME = 'axolcode-generated';

/** Serves the emitted Rust output through a read-only custom-scheme document. */
export class GeneratedRustProvider implements vscode.TextDocumentContentProvider {
  private readonly contents = new Map<string, string>();
  private readonly changeEmitter = new vscode.EventEmitter<vscode.Uri>();

  public readonly onDidChange = this.changeEmitter.event;

  /** Returns the cached emitted Rust content for the requested virtual document. */
  public provideTextDocumentContent(uri: vscode.Uri): string {
    return this.contents.get(uri.path) ?? '';
  }

  /** Stores new content for a virtual document and refreshes it when open. */
  public set(uri: vscode.Uri, content: string): void {
    this.contents.set(uri.path, content);
    this.changeEmitter.fire(uri);
  }

  /** Releases the change event emitter. */
  public dispose(): void {
    this.changeEmitter.dispose();
  }
}

/** Builds the virtual document URI for the Rust emitted from an .axol source file. */
export function generatedRustUri(sourcePath: string): vscode.Uri {
  const normalized = sourcePath.replace(/\\/g, '/').replace(/^\//, '');
  return vscode.Uri.parse(`${GENERATED_RUST_SCHEME}:/${normalized}.rs`);
}
