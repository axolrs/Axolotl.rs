// Owner: PascalElixir / axolrs (GitHub org)
// File: Gills (axol-analyzer) language client construction for the axol language.

import * as vscode from 'vscode';
import {
  LanguageClient,
  type DocumentSelector,
  type LanguageClientOptions,
  type ServerOptions,
} from 'vscode-languageclient/node';
import type { ServerExecutable } from './server';

export const CLIENT_ID = 'axolcode';
export const CLIENT_NAME = 'AxolCode (Gills)';

export const AXOL_DOCUMENT_SELECTOR: DocumentSelector = [
  { language: 'axol', scheme: 'file' },
  { language: 'axol', scheme: 'untitled' },
];

/** Creates the language client that spawns the Gills server and speaks LSP over stdio. */
export function createLanguageClient(
  server: ServerExecutable,
  outputChannel: vscode.LogOutputChannel,
  cwd: string | undefined,
): LanguageClient {
  const serverOptions: ServerOptions = {
    command: server.command,
    args: [...server.args],
    options: cwd !== undefined ? { cwd } : undefined,
  };
  const clientOptions: LanguageClientOptions = {
    documentSelector: AXOL_DOCUMENT_SELECTOR,
    outputChannel,
  };
  return new LanguageClient(CLIENT_ID, CLIENT_NAME, serverOptions, clientOptions);
}
