/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: CodeMirror 6 wiring for the playground - an always-dark editor theme, an Axolotl highlight palette, and the editor factory with Mod-Enter bound to Run.
*/

import { basicSetup } from 'codemirror';
import { StreamLanguage, HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { EditorView, keymap } from '@codemirror/view';
import { Prec, type Extension } from '@codemirror/state';
import { indentWithTab } from '@codemirror/commands';
import { tags as t } from '@lezer/highlight';
import { axolMode } from './axol-mode';

const axolHighlight = HighlightStyle.define([
	{ tag: t.keyword, color: '#ff7b95' },
	{ tag: t.atom, color: '#d2a8ff' },
	{ tag: t.string, color: '#7ee787' },
	{ tag: t.number, color: '#d2a8ff' },
	{ tag: t.comment, color: '#8b949e', fontStyle: 'italic' },
	{ tag: t.typeName, color: '#ffa657' },
	{ tag: t.operator, color: '#d0d7de' },
	{ tag: t.punctuation, color: '#8b949e' },
]);

const axolTheme = EditorView.theme(
	{
		'&': { color: '#e6edf3', backgroundColor: 'transparent', height: '100%' },
		'.cm-scroller': { fontFamily: 'var(--font-mono)', lineHeight: '1.65' },
		'.cm-content': { caretColor: '#ff7b95', paddingBottom: '1.5rem' },
		'.cm-cursor': { borderLeftColor: '#ff7b95', borderLeftWidth: '2px' },
		'&.cm-focused .cm-selectionBackground, .cm-selectionBackground': {
			backgroundColor: 'rgba(255, 123, 149, 0.22)',
		},
		'.cm-gutters': { backgroundColor: 'transparent', color: '#6e7681', border: 'none' },
		'.cm-activeLine': { backgroundColor: 'rgba(255, 255, 255, 0.04)' },
		'.cm-activeLineGutter': { backgroundColor: 'rgba(255, 255, 255, 0.06)', color: '#e6edf3' },
		'.cm-lineNumbers .cm-gutterElement': { padding: '0 0.75rem 0 1rem', minWidth: '2.25rem' },
		'.cm-matchingBracket': {
			backgroundColor: 'rgba(255, 255, 255, 0.08)',
			outline: '1px solid rgba(255, 123, 149, 0.45)',
		},
		'.cm-tooltip': { border: '1px solid #30363d', backgroundColor: '#161b22' },
	},
	{ dark: true }
);

export function createAxolEditor(parent: HTMLElement, doc: string, onRun: () => void): EditorView {
	const runBinding = Prec.highest(
		keymap.of([
			{
				key: 'Mod-Enter',
				run: () => {
					onRun();
					return true;
				},
			},
		])
	);
	const extensions: Extension[] = [
		basicSetup,
		keymap.of([indentWithTab]),
		StreamLanguage.define(axolMode),
		syntaxHighlighting(axolHighlight),
		axolTheme,
		runBinding,
	];
	return new EditorView({ doc, parent, extensions });
}

export function replaceDocument(view: EditorView, doc: string): void {
	view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: doc } });
}
