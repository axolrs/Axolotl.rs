// Owner: PascalElixir / axolrs (GitHub org)
// File: Axolotl status bar item with starting, ready, busy, and failure states.

import * as vscode from 'vscode';

const STATUS_PREFIX = 'Axolotl:';

/** Manages the 'Axolotl: <status>' status bar item shown in the bottom bar. */
export class AxolStatusBar implements vscode.Disposable {
  private readonly item: vscode.StatusBarItem;
  private hiddenByUser = false;

  constructor() {
    this.item = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    this.item.command = 'axolcode.buildStatus';
    this.item.name = 'AxolCode build status';
    this.setStarting();
    this.item.show();
  }

  /** Shows a spinning state used while the Gills language server starts. */
  public setStarting(): void {
    this.item.text = `$(sync~spin) ${STATUS_PREFIX} starting`;
    this.item.tooltip = 'AxolCode is starting the Gills (axol-analyzer) language server.';
    this.item.color = undefined;
    this.item.backgroundColor = undefined;
    this.refreshVisibility();
  }

  /** Shows the ready state used after the language server has initialized. */
  public setReady(): void {
    this.item.text = `$(check) ${STATUS_PREFIX} ready`;
    this.item.tooltip = 'Gills (axol-analyzer) is connected. Click to toggle this item.';
    this.item.color = undefined;
    this.item.backgroundColor = undefined;
    this.refreshVisibility();
  }

  /** Shows a dimmed spinning state used while a tool command runs in a terminal. */
  public setBusy(label: string): void {
    this.item.text = `$(sync~spin) ${STATUS_PREFIX} ${label}`;
    this.item.tooltip = 'An Axolotl tool command is running in a terminal.';
    this.item.color = new vscode.ThemeColor('statusBarItem.offlineForeground');
    this.item.backgroundColor = undefined;
    this.refreshVisibility();
  }

  /** Shows a failure state with error coloring. */
  public setFailed(label: string): void {
    this.item.text = `$(error) ${STATUS_PREFIX} ${label}`;
    this.item.tooltip = 'The last Axolotl tool command reported a failure.';
    this.item.color = new vscode.ThemeColor('statusBarItem.errorForeground');
    this.item.backgroundColor = new vscode.ThemeColor('statusBarItem.errorBackground');
    this.refreshVisibility();
  }

  /** Shows the state used when neither server command could be found. */
  public setServerMissing(): void {
    this.item.text = `$(warning) ${STATUS_PREFIX} server not found`;
    this.item.tooltip = 'AxolCode could not start Gills. Check the axolcode.server.path setting.';
    this.item.color = new vscode.ThemeColor('statusBarItem.warningForeground');
    this.item.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
    this.refreshVisibility();
  }

  /** Shows the state used after the language server has stopped. */
  public setStopped(): void {
    this.item.text = `$(circle-slash) ${STATUS_PREFIX} stopped`;
    this.item.tooltip = 'The Gills language server is stopped.';
    this.item.color = undefined;
    this.item.backgroundColor = undefined;
    this.refreshVisibility();
  }

  /** Toggles the item between hidden and visible, revealing it when hidden. */
  public toggle(): void {
    this.hiddenByUser = !this.hiddenByUser;
    this.refreshVisibility();
  }

  /** Releases the underlying status bar resource. */
  public dispose(): void {
    this.item.dispose();
  }

  /** Applies the user visibility preference to the underlying status bar item. */
  private refreshVisibility(): void {
    if (this.hiddenByUser) {
      this.item.hide();
    } else {
      this.item.show();
    }
  }
}
