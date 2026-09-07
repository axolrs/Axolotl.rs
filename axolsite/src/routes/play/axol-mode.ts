/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: CodeMirror stream tokenizer for Axolotl - keywords, built-in and user types, interpolated strings, char literals, numbers, and both comment forms.
*/

import type { StreamParser } from '@codemirror/language';

const KEYWORDS = new Set([
	'let',
	'var',
	'fn',
	'end',
	'if',
	'then',
	'elseif',
	'else',
	'while',
	'do',
	'repeat',
	'until',
	'for',
	'in',
	'break',
	'continue',
	'return',
	'match',
	'case',
	'struct',
	'enum',
	'interface',
	'use',
	'pub',
	'const',
	'and',
	'or',
	'not',
	'spawn',
	'await',
	'async',
	'try',
	'catch',
	'unsafe',
	'move',
	'borrow',
	'type',
	'where',
	'as',
	'is',
	'extern',
	'loop',
	'self',
]);

const ATOMS = new Set(['true', 'false', 'nil', 'null']);

const OPERATOR = /<<=|>>=|<<|>>|->|=>|==|!=|~=|<=|>=|\+=|-=|\*=|\/=|%=|&=|\|=|\^=|\.\.\.|\.\.|\|>|::|\?\.|\?\?|!!/;
const SINGLE_OPERATOR = /^[+\-*/%<>=!&|^~?]/;
const PUNCTUATION = /^[()[\]{},;:.@$#]/;
const NUMBER = /^(?:0[xX][0-9a-fA-F_]+|\d[\d_]*(?:\.[\d_]+)?(?:[eE][+-]?\d+)?)/;
const IDENTIFIER = /^[A-Za-z_][A-Za-z0-9_]*/;

interface AxolStreamState {
	inBlockComment: boolean;
	inString: boolean;
}

export const axolMode: StreamParser<AxolStreamState> = {
	name: 'axol',
	startState: () => ({ inBlockComment: false, inString: false }),
	token: (stream, state) => {
		if (state.inBlockComment) {
			if (!stream.match('--]]')) stream.skipToEnd();
			else state.inBlockComment = false;
			return 'comment';
		}
		if (state.inString) {
			const matched = stream.match(/(?:\\.|[^"\\])*"?/);
			if (matched && stream.pos > stream.start) {
				if (stream.current().endsWith('"')) state.inString = false;
				return 'string';
			}
			stream.next();
			return 'string';
		}
		if (stream.eatSpace()) return null;
		if (stream.match('--[[')) {
			state.inBlockComment = true;
			return 'comment';
		}
		if (stream.match(/--[^\n]*/)) return 'comment';
		const string = stream.match(/"(?:\\.|[^"\\])*"?/);
		if (string) {
			if (!stream.current().endsWith('"')) state.inString = true;
			return 'string';
		}
		if (stream.match(/'(?:\\.|[^'\\])*'?/)) return 'string';
		if (stream.match(NUMBER)) return 'number';
		const word = stream.match(IDENTIFIER);
		if (word) {
			const name = stream.current();
			if (KEYWORDS.has(name)) return 'keyword';
			if (ATOMS.has(name)) return 'atom';
			if (/^[A-Z]/.test(name)) return 'typeName';
			return 'variableName';
		}
		if (stream.match(OPERATOR) || stream.match(SINGLE_OPERATOR)) return 'operator';
		if (stream.match(PUNCTUATION)) return 'punctuation';
		stream.next();
		return null;
	},
};
